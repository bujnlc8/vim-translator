# 一个有用的翻译 VIM 插件, 提供百度和腾讯两种选择

**⚠️ 由于有道翻译更改了签名的方法导致接口调用出错，懒得去逆向了，因此移除有道翻译，新增了更友好的腾讯翻译。如果在配置文件指定了翻译渠道为`youdao`，请更换成`baidu`或`tencent`**

先上图：

![](https://s3.bmp.ovh/imgs/2021/10/9e32b13bc6936ca8.jpg)

## 安装

- 将代码 clone 到`~/.vim/plugin`下面即可。

OR

- 如果你有安装插件管理工具，比如, `vim-plug`, 可以加入以下行到你的`.vimrc`进行安装

<!---->

    Plug 'bujnlc8/vim-translator'

**只在 macOS 中测试过，理论上 Linux 也适用，Windows 应该会报错**

## 命令

- `:Ti`, 支持在底部输入框输入翻译，可以在`.vimrc`加入`noremap <leader>ti :<C-u>Ti<CR>`支持快捷键输入。

- `:Ty`, 从粘贴板中获取文字进行翻译(匿名寄存器中""), 可以在`.vimrc`加入`noremap <leader>ty :<C-u>Ty<CR>`支持快捷键输入。

- `:Tc`, 支持翻译光标处单词，可以在`.vimrc`加入`noremap <leader>tc :<C-u>Tc<CR>`支持快捷键。

- `:Tv`, 支持在 visual 模式下选中翻译，可以在`.vimrc`加入`vnoremap <leader>tv :<C-u>Tv<CR>`支持快捷键。

- `:Tr`, 支持在 visual 模式下将文字替换成翻译，可以在`.vimrc`加入`vnoremap <leader>tr :<C-u>Tr<CR>`支持快捷键。

- `:Te`, 收藏单词或语句，如果提供了一个参数，那么会收藏该参数，否则收藏光标处的单词。

- `:Tev`, 收藏 visual 模式下选中的词汇，需要在 visual 模式下启用。

- `:Tee`, 编辑收藏的单词或语句，可以像编辑一个文本来进行编辑。

- `:Tz`, 查询中文拼音及释义等。

## 选项

- `let g:translator_cache=1`， 是否启用缓存，默认 1。

- `let g:translator_cache_path='~/.cache'`，缓存路径，默认`expand('<sfile>:p:h').'/.cache'`。

- `let g:translator_channel='baidu'`，查询通道，默认`baidu`, 也可切到`tencent`。

- `let g:translator_outputype='popup'`, 结果输出方式，如果支持弹窗（vim-8.2 及以上）默认弹窗展示，否则输出到底部，也可以设置成`echo`显式开启输出到底部，弹窗模式下按`z`关闭弹窗。`echo` 模式下，如果结果长度大于 200，会输出到`fixquick`窗口中。

## 后记

在写插件的过程中借鉴了[vim-youdao-translater](https://github.com/ianva/vim-youdao-translater) 这个项目，特此表示感谢 ♥️。

在使用过程中如果出现问题，欢迎在 Issues 提出。

另外，为了推广 python3, 本插件只支持 python3。

如果百度翻译提示 token 失效或者类似的错误，请尝试用 chrome 访问[百度翻译](https://fanyi.baidu.com/)，打开开发者工具，随意输入一个单词查询，在`https://fanyi.baidu.com/v2transapi?xxx`的 form 表单提交中应该有一个`token`字段，在 Cookies 中应该有一个名字为`BAIDUID_BFESS`的 cookie, 将它们的值分别替换掉`plugin/baidu.py`文件顶部的`TOKEN`和`BAIDUID_BFESS`变量。 替换之后再次尝试。
