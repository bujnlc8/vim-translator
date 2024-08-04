use anyhow::{anyhow, Result};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use colored::Colorize;
use qiniu_uploader::{QiniuRegionEnum, QiniuUploader};
use qrcode::{render::unicode, QrCode};
use std::{
    io::{self},
    os::unix::fs::MetadataExt,
    str::FromStr,
    time,
};
use std::{path::PathBuf, process::exit};
use tokio::{fs::File, io::AsyncRead};

pub async fn upload_to_qiniu<R: AsyncRead + Send + Sync + 'static + std::marker::Unpin>(
    reader: R,
    access_key: &str,
    secret_key: &str,
    bucket_name: &str,
    object_name: &str,
    file_size: usize,
    region: Option<QiniuRegionEnum>,
    part_size: Option<usize>,
    threads: Option<u8>,
) -> Result<()> {
    let qiniu = QiniuUploader::new(
        access_key.to_string(),
        secret_key.to_string(),
        bucket_name.to_string(),
        region,
        false,
    );
    qiniu
        .part_upload_file(object_name, reader, file_size, part_size, threads, None)
        .await?;
    Ok(())
}

#[derive(Parser)]
#[clap(version, about, long_about=None)]
pub struct Cli {
    /// 七牛access key，或自动从环境变量 `QINIU_ACCESS_KEY` 获取
    #[clap(short, long)]
    access_key: Option<String>,
    /// 七牛secret key, 或自动从环境变量 `QINIU_SECRET_KEY` 获取
    #[clap(short, long)]
    secret_key: Option<String>,
    /// 对象名称，如果未指定会从`file_path`参数解析，一般不建议设置
    #[clap(short, long)]
    object_name: Option<String>,
    /// 文件绝对路径
    #[clap(short, long)]
    file_path: Option<PathBuf>,
    /// 七牛bucket名称
    #[clap(short, long)]
    bucket_name: Option<String>,
    /// 七牛bucket region，如z0，华东-浙江(默认)，详见 https://developer.qiniu.com/kodo/1671/region-endpoint-fq
    #[clap(long)]
    region: Option<String>,
    /// 下载域名，需要和bucket匹配，如果设置，会显示下载链接及输出二维码
    #[clap(short, long)]
    domain_name: Option<String>,
    /// 生成shell补全脚本, 支持Bash, Zsh, Fish, PowerShell, Elvish
    #[arg(long)]
    completion: Option<String>,
    /// 不要输出下载链接二维码
    #[clap(long, action)]
    no_qrcode: bool,
    /// 分片上传的大小，单位bytes，1M-1GB之间，如果指定，优先级比threads参数高
    #[arg(long)]
    part_size: Option<usize>,
    /// 分片上传线程，在未指定part_size参数的情况下生效，默认5
    #[arg(long)]
    threads: Option<u8>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let start = time::Instant::now();
    if let Some(shell) = cli.completion {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();
        match Shell::from_str(&shell.to_lowercase()) {
            Ok(shell) => generate(shell, &mut cmd, bin_name, &mut io::stdout()),
            Err(e) => {
                return Err(anyhow!("{}", e.to_string()));
            }
        };
        return Ok(());
    }
    let qiniu_access_key = match cli.access_key {
        Some(key) => key,
        None => match std::env::var("QINIU_ACCESS_KEY") {
            Ok(key) => key,
            Err(_) => {
                eprintln!("{}", "Qiniu access_key 为空！".red());
                exit(1)
            }
        },
    };
    let qiniu_secret_key = match cli.secret_key {
        Some(key) => key,
        None => match std::env::var("QINIU_SECRET_KEY") {
            Ok(key) => key,
            Err(_) => {
                eprintln!("{}", "Qiniu secret_key 为空！".red());
                exit(1)
            }
        },
    };
    let file_path = cli.file_path.unwrap_or_else(|| {
        eprintln!("{}", "file-path is required".red());
        exit(1);
    });

    let bucket_name = cli.bucket_name.unwrap_or_else(|| {
        eprintln!("{}", "bucket-name is required".red());
        exit(1);
    });
    let file = File::open(&file_path).await.unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!("read {} failed！", file_path.to_str().unwrap()).red()
        );
        exit(1);
    });
    let object_name = match cli.object_name {
        Some(name) => name,
        None => {
            format!(
                "uploads/{}",
                file_path.file_name().unwrap().to_str().unwrap()
            )
        }
    };
    // size in bytes
    let size = file.metadata().await.unwrap().size();
    let region = QiniuRegionEnum::from_str(&cli.region.unwrap_or("z0".to_string())).unwrap();
    match upload_to_qiniu(
        file,
        &qiniu_access_key,
        &qiniu_secret_key,
        &bucket_name,
        object_name.as_str(),
        size as usize,
        Some(region),
        cli.part_size,
        cli.threads,
    )
    .await
    {
        Ok(()) => {
            println!("{}", format!("🚀 upload {} success！", object_name).green())
        }
        Err(e) => {
            eprintln!(
                "{}",
                format!("😭 upload {} failed: {}！", object_name, e).red(),
            );
        }
    };
    let download_url = match cli.domain_name {
        Some(domain_name) => {
            if domain_name.starts_with("http") {
                format!("{domain_name}/{object_name}")
            } else {
                format!("https://{domain_name}/{object_name}")
            }
        }
        None => "".to_string(),
    };
    if !download_url.is_empty() {
        println!("🔗 {}", download_url.yellow());
        if !cli.no_qrcode {
            let code = QrCode::new(download_url).unwrap();
            let image = code
                .render::<unicode::Dense1x2>()
                .module_dimensions(1, 1)
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!("{}", image);
        }
    }
    println!(
        "{}",
        format!("{:.2}s elapsed.", start.elapsed().as_secs_f64()).cyan()
    );
    Ok(())
}
