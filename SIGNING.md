# Mac 签名 / 公证 + Windows 签名 配置指南

> 目标：让 `.dmg`(Mac) / `.msi`/`.exe`(Windows) 安装时不弹"无法验证开发者 / SmartScreen"。
> CI 在 `.github/workflows/release.yml`,推 tag `v*` 触发。

## 已知信息(从 `ashot-mobile` 拿到)
- **Apple Team ID:** `2974J97HMX`(公司账号,iOS `com.ggt.ashot` 在用)
- 桌面 bundle id:`live.ashot.creator.desktop`(Developer ID 公证不要求预注册 App ID,无需对齐 iOS 的 `com.ggt.ashot`)

## Mac:需要一张 "Developer ID Application" 证书
⚠️ 这跟 iOS 的 "Apple Development / Apple Distribution" 是**不同类型**。store 外分发必须用 Developer ID Application。
若开发者后台 Certificates 里还没有:
1. Apple Developer 后台 → Certificates → ＋ → **Developer ID Application**(仅 Account Holder 可建)。
2. 本机钥匙串生成 CSR → 下载 .cer → 双击导入钥匙串。
3. 钥匙串里右键该证书 → 导出为 **.p12**(设个密码)。

### 公证用凭据(二选一)
- **App-specific password**:appleid.apple.com → 登录与安全 → App 专用密码,生成一个。
- 或 **App Store Connect API Key**(.p8 + key id + issuer id)。

## GitHub Secrets(在仓库 Settings → Secrets and variables → Actions 添加)
| Secret | 值 | 说明 |
|---|---|---|
| `APPLE_CERTIFICATE` | `base64 -i DeveloperID.p12 \| pbcopy` 的结果 | .p12 的 base64 |
| `APPLE_CERTIFICATE_PASSWORD` | 导出 .p12 时设的密码 | |
| `APPLE_SIGNING_IDENTITY` | `Developer ID Application: <公司名> (2974J97HMX)` | 钥匙串里证书的完整名称 |
| `APPLE_ID` | 公司 Apple ID 邮箱 | |
| `APPLE_PASSWORD` | 上面的 App 专用密码 | |
| `APPLE_TEAM_ID` | `2974J97HMX` | |

> 查 `APPLE_SIGNING_IDENTITY` 完整名:`security find-identity -v -p codesigning`(导入证书后)。

## 本机签名(可选,不走 CI 时)
```bash
export APPLE_SIGNING_IDENTITY="Developer ID Application: <公司名> (2974J97HMX)"
export APPLE_ID="<apple-id>"
export APPLE_PASSWORD="<app-专用密码>"
export APPLE_TEAM_ID="2974J97HMX"
export PATH="$HOME/.cargo/bin:$PATH"
npm run tauri build     # Tauri 自动签名 + 公证 .dmg
```

## Windows 代码签名(可选,后补)
不签也能装,但首次会弹 SmartScreen。要去掉需 Authenticode 证书(OV/EV)。
对应 secrets(tauri-action 支持):`WINDOWS_CERTIFICATE`(.pfx base64)、`WINDOWS_CERTIFICATE_PASSWORD`。
拿到后我再把 `release.yml` 的 Windows job 加上签名步骤。
