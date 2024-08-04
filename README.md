# 七牛文件上传工具

基于[https://github.com/bujnlc8/qiniu-uploader](https://github.com/bujnlc8/qiniu-uploader)

支持显示上传进度条及输出下载链接二维码

![](./snapshots/help.png)

![](./snapshots/snapshot.png)

## 下载

- [⬇️ 点击下载 x86_64-apple-darwin](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_x86_64-apple-darwin.tar.gz)

  > [qiniu-upload-util_x86_64-apple-darwin.tar.gz.md5](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_x86_64-apple-darwin.tar.gz.md5)

- [⬇️ 点击下载 aarch64-apple-darwin](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_aarch64-apple-darwin.tar.gz)

  > [qiniu-upload-util_aarch64-apple-darwin.tar.gz.md5](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_aarch64-apple-darwin.tar.gz.md5)

- [⬇️ 点击下载 x86_64-unknown-linux-musl](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_x86_64-unknown-linux-musl.tar.gz)

  > [qiniu-upload-util_x86_64-unknown-linux-musl.tar.gz.md5](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_x86_64-unknown-linux-musl.tar.gz.md5)

- ~~[⬇️ 点击下载 x86_64-unknown-linux-gnu](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_x86_64-unknown-linux-gnu.tar.gz)~~

  > ~~[qiniu-upload-util_x86_64-unknown-linux-gnu.tar.gz.md5](https://github.com/bujnlc8/qiniu-upload-util/releases/download/0.1.0/qiniu-upload-util_x86_64-unknown-linux-gnu.tar.gz.md5)~~

请根据你的操作系统下载相应的版本，可对比 md5 hash 值确定是否下载了最新的版本

解压后运行，在 Macos 中如果出现`"qiniu-upload-util" is damaged and can't beopened.`的提示，请尝试执行以下命令:

```
sudo spctl --master-disable
```

**在 Arm 版的 Mac 上如果仍然打不开，可以尝试 x86 的版本**

## 编译

```
cargo build --release --locked
```

**如果在使用过程中发现 bug，欢迎反馈 👏**
