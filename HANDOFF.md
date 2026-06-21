# ASHOT Creator Studio — 桌面客户端交接

**项目路径:** `/Users/justinopenclaw/projects/ashot-creator-desktop`
**目标:** Tauri 2 桌面壳,壳套 `https://creator.ashot.live/`,出 **Windows + Mac** 安装包,让创作者本地安装运行。AI 生成仍走我们服务器 API(CLIPROXY),文件可存创作者本地。

## 当前状态(已完成)
- 用官方 `create-tauri-app`(vanilla 模板)生成 Tauri 2 脚手架,已从主仓库 `ashot-creator-mvp/` 移到本独立目录。
- 结构:`package.json` · `src/`(index.html/main.js/styles.css)· `src-tauri/`(Cargo.toml / build.rs / tauri.conf.json / capabilities/ / icons/ / src/)。
- **尚未定制**:`src-tauri/tauri.conf.json` 还是默认值(productName "desktop")。
- **本机 Rust 未安装**(Tauri 构建需要)。

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
