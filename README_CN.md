# Vite Tauri 桌面应用程序模板

对于那些想要使用 Web 技术制作桌面应用程序的人来说，这是一个入门模板。 该模板使用以下技术栈

### Vite.js
Vite.js 是一个新的现代 JavaScript 捆绑器，速度极快，并且包含许多合理的默认值。

### Tauri
Tauri 是一项新的现代技术，可将您的 Web 应用程序转变为适用于多个平台（Windows、MacOS、Linux、android 和 ios）的桌面应用程序。 Tauri 应用程序的文件大小非常小，内存消耗也很小。

### Vue 3
Vue.js 是一个增量前端框架，使用它绝对是一种乐趣。 它在3.0中得到了非常令人印象深刻的改进，包括 Composition Api、脚本设置、动态 CSS 绑定等等。

### [Vuetify 3](https://vuetifyjs.com/)
Vuetify 可以说是 Vue 3 的最佳组件库，目前处于 alpha 阶段，但很快就会投入生产。 许多预制组件将使您作为应用程序开发人员的工作变得更轻松、更有趣。

[字体](https://vuetifyjs.com/en/features/icon-fonts/)

## 安装
- 根据Tauri准备好你的工作空间。[Tauri入门](https://tauri.app/v1/guides/getting-started/prerequisites/)

  - 注意：您只需要安装全局性的东西，如Rust和其他操作系统级别的软件包。任何和应用程序本身相关的东西都已经安装好了，可以随时使用。

- 克隆本仓库
  - `git clone https://github.com/yooneskh/vite-tauri-template app-name`

- `yarn` (或 `npm install`，但yarn是更好的的选择)

- 根据您的应用程序修改以下文件

  - ./index.html
  - ./package.json
  - ./public/favicon.svg
  - ./src-tauri/icons/*
  - ./src-tauri/tauri.conf.json

## 开发

有两种方法可以开发你的应用程序

### In Browser
- `yarn serve`
  - launches vite and you can test and develop your app in the browser at http://localhost:8080.

### 浏览器中

用以下两个命令分别启动一个终端

1- `yarn serve:tauri`

这将启动Vite并配置[统一网络](https://github.com/yooneskh/unified-network)（这是我的）使用Tauri进行api调用（以解决CORS问题）。

2- `yarn serve:native`

这将启动Tauri窗口，您将在本地窗口中看到您的应用程序。

**注意:** 浏览器和Tauri窗口的开发主要有2个区别

- 一是谁执行你的http调用，因为在浏览器中，你受到CORS规则的约束，但在Tauri模式下测试时，Tauri的本机模块正在执行http调用，所以CORS不会成为问题。

- 其次是渲染引擎。 在浏览器中，它通常是最新的现代引擎，但在 Tauri 中，它将是操作系统的 Web 引擎，这很好，但可能不如浏览器好。

3- 调试

Vue - [安装devtools](https://blog.csdn.net/qq_37331806/article/details/126319661)

运行这个命令

`cnpm install -g @vue/devtools`
 
或运行这个

`npm install -g @vue/devtools`

安装后并在cmd中运行此命令

`vue-devtools`

然后将窗口中的代码复制到index.html文件中


## Building

`yarn build` 构建 Web 应用程序并将其与 Tauri ("./src-tauri/target/releases") 一起打包 

## License
用它做任何你想做的事！