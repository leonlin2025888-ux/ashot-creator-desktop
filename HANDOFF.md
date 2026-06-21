# ASHOT Creator Studio — 桌面客户端交接

**项目路径:** `/Users/justinopenclaw/projects/ashot-creator-desktop`
**目标:** Tauri 2 桌面壳,壳套 `https://creator.ashot.live/`,出 **Windows + Mac** 安装包,让创作者本地安装运行。AI 生成仍走我们服务器 API(CLIPROXY),文件可存创作者本地。

## 当前状态(2026-06-20 已推进)
- ✅ Tauri 2 脚手架(vanilla 模板),独立目录。
- ✅ `src-tauri/tauri.conf.json` 已定制:productName `ASHOT Creator Studio`、identifier `live.ashot.creator.desktop`、主窗口 `label:"main"` 直接 `url` 指向 `https://creator.ashot.live/`、targets `app/dmg/nsis/msi`、category Productivity。
- ✅ 品牌图标已替换(`tauri icon` 用 `ashot-creator-mvp/public/brand/app-icon-ios-1024.png` 生成 icns/ico/全平台 png)。
- ✅ 品牌加载页:`src/`(index.html/styles.css/main.js)改成深色 + 橙红渐变(#FF7A26→#FF3D2E)logo + 加载动画 + 离线提示;移除了 greet 样板。
- ✅ Rust 工具链已装(cargo/rustc 1.96.0,经 **USTC 镜像** rustup;`~/.cargo/config.toml` 配 USTC crates 镜像加速依赖)。
- ✅ **Mac 包已构建成功**:`src-tauri/target/release/bundle/` 下 `ASHOT Creator Studio.app`(8.5M)+ `ASHOT Creator Studio_0.1.0_aarch64.dmg`(3.2M)。已 `open` 验证可启动、无崩溃。
- ✅ GitHub Actions CI:`.github/workflows/release.yml`(macOS 通用包 + 签名公证 secrets;Windows msi/nsis;推 tag `v*` 触发)。
- ✅ `git init` + 首次提交完成(72 文件)。
- ⬜ **未做**:Mac 签名/公证(需真实 Apple 证书 secrets,见下)、Windows 包(只能 CI 出)、推到 GitHub 远程触发 CI。

### 国内网络踩坑记录(重要)
- 官方 rust-lang.org / rsproxy.cn 的 rustup-init 下载会卡死(<10B/s)。→ 用 **USTC**:`https://mirrors.ustc.edu.cn/rust-static/rustup/dist/aarch64-apple-darwin/rustup-init`,`RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static`。
- rsproxy crates 镜像构建时也会超时。→ `~/.cargo/config.toml` 用 USTC:`sparse+https://mirrors.ustc.edu.cn/crates.io-index/`。
- npm 首装 Tauri 原生二进制会损坏(npm 可选依赖 bug)。→ 删 node_modules+package-lock,`npm install --include=optional`,registry 用 `registry.npmmirror.com`。
- `brew install rust` 依赖 llvm(~1.5G)且 ghcr 下载失败,**别用**,用 rustup+USTC。

### 旧版历史(已不适用,留档)
- 原:productName 默认 "desktop",本机 Rust 未安装。

## 关键决策(已和用户确认)
- **形态 = 方案 A**:Tauri 壳加载 creator.ashot.live(快速 v1),后续再演进本地存储/本地优先。
- **Apple 证书已有**:公司 iOS 已发布,Mac 签名/公证**复用现有 Apple Developer 账号**(Developer ID Application + notarytool),不另买。
- **Tauri 而非 Electron**(安装包小、省内存)。
- 登录:复用已建的**创作者 API Key**(`/creator/api` 自助页)或网页登录。

## 待办(按顺序)

### 1. 装 Rust(构建前置)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

### 2. 定制 `src-tauri/tauri.conf.json`
把 productName / identifier / 主窗口改成下面这份(主窗口直接 `url` 指向远程站):
```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "ASHOT Creator Studio",
  "version": "0.1.0",
  "identifier": "live.ashot.creator.desktop",
  "build": { "frontendDist": "../src" },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "ASHOT Creator Studio",
        "url": "https://creator.ashot.live/",
        "width": 1280, "height": 832, "minWidth": 960, "minHeight": 600,
        "resizable": true, "fullscreen": false
      }
    ],
    "security": { "csp": null }
  },
  "bundle": {
    "active": true,
    "targets": ["app", "dmg", "nsis", "msi"],
    "icon": ["icons/32x32.png","icons/128x128.png","icons/128x128@2x.png","icons/icon.icns","icons/icon.ico"],
    "category": "Productivity",
    "copyright": "© ASHOT"
  }
}
```
⚠️ `identifier` 要对齐 Apple Developer 里登记的 App ID(没有就在开发者后台建一个,或用通配 App ID),否则 Mac 签名会失败。

### 3. 用品牌图标替换占位图标
源图:`../ashot-creator-mvp/public/brand/app-icon-ios-1024.png`(1024×1024)。
```bash
cd /Users/justinopenclaw/projects/ashot-creator-desktop
npm install
npx tauri icon ../ashot-creator-mvp/public/brand/app-icon-ios-1024.png   # 自动生成各平台 icns/ico/png
```

### 4. 本地跑 + 出 Mac 包
```bash
npm run tauri dev      # 开发预览(加载 creator.ashot.live)
npm run tauri build    # 出 .app / .dmg(Mac);Windows 目标在 Mac 上不产出,见 CI
```

### 5. Mac 签名 + 公证(复用现有 Apple 证书)
环境变量(用公司现有 Developer ID Application 证书 + App Store Connect API key / app-specific password):
```bash
export APPLE_SIGNING_IDENTITY="Developer ID Application: <公司名> (<TEAMID>)"
export APPLE_ID="<apple-id>"
export APPLE_PASSWORD="<app-specific-password>"   # 或用 APPLE_API_KEY / APPLE_API_ISSUER
export APPLE_TEAM_ID="<TEAMID>"
npm run tauri build    # Tauri 会自动签名 + 公证 .dmg
```

### 6. GitHub Actions CI 出双平台安装包(关键——Windows 不能在 Mac 上构建)
新建 `.github/workflows/release.yml`:macos-latest 出 .dmg(带签名公证 secrets)+ windows-latest 出 .msi/.nsis,推 tag 触发,产物上传 release。用 `tauri-apps/tauri-action`。
- Mac secrets:`APPLE_CERTIFICATE`(base64 of .p12)、`APPLE_CERTIFICATE_PASSWORD`、`APPLE_SIGNING_IDENTITY`、`APPLE_ID`、`APPLE_PASSWORD`、`APPLE_TEAM_ID`。
- Windows 签名(可选):代码签名证书,否则装时弹 SmartScreen(可后补)。

### 7. git init
```bash
cd /Users/justinopenclaw/projects/ashot-creator-desktop && git init && git add -A && git commit -m "init: ASHOT Creator Studio Tauri shell"
```

## 后续(v2,验证需求后再做)
- **本地存盘**:接今天上线的 `?response_format=b64_json` 一次性交付 API——App 生成→拿 base64→写创作者本地文件夹→服务器 24h 自动清。需要 Tauri command(Rust 侧写文件)+ 远程站 IPC 授权(`dangerousRemoteDomainIpcAccess` 限定 creator.ashot.live)。
- 原生菜单 / 深链 / 自动更新(Tauri updater)。

## 相关(主仓库这边今天已上线的配套 API)
- base64 一次性交付 + 24h purge(`GET /api/creator/v1/image-generations/{id}?response_format=b64_json`)
- 对外限流提升:chat/responses 6000/h、image 1000/h
- 失败带原因(`code: GENERATION_FAILED` + `error`)、生图误用参数回 `warnings`
- 对接文档:`ashot-creator-mvp/docs/THIRD_PARTY_API.md`(也在 `/creator/api` 页)
