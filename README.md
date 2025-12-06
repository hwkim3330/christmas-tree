# 🎄 Shared Christmas Tree

> 친구, 가족과 함께 크리스마스 트리를 꾸며보세요!

실시간으로 동기화되는 공유 크리스마스 트리 웹앱입니다. 트리를 클릭하고 이름을 입력하면 오너먼트(장식)가 추가됩니다.

## 🌐 Live Demo

**[https://hwkim3330.github.io/christmas-tree/](https://hwkim3330.github.io/christmas-tree/)**

### 버전 선택

| Version | URL | Description | Storage | Size |
|---------|-----|-------------|---------|------|
| **Standard** | [index.html](https://hwkim3330.github.io/christmas-tree/) | 풀 기능 버전 (눈, 별, 집, 조명, 선물) | Firebase | ~48KB |
| **WASM** | [wasm.html](https://hwkim3330.github.io/christmas-tree/wasm.html) | WebAssembly 고성능 버전 | Firebase | ~85KB |
| **Lite** | [lite.html](https://hwkim3330.github.io/christmas-tree/lite.html) | 경량 버전 (빠른 로딩) | Firebase | ~9KB |
| **Local** | [local.html](https://hwkim3330.github.io/christmas-tree/local.html) | 오프라인 개인용 | LocalStorage | ~22KB |
| **Embed** | [embed.html](https://hwkim3330.github.io/christmas-tree/embed.html) | iframe 삽입용 (읽기 전용) | Firebase | ~6KB |
| **Kiosk** | [kiosk.html](https://hwkim3330.github.io/christmas-tree/kiosk.html) | 디지털 사이니지/전시용 | Firebase | ~12KB |
| **Mini** | [mini.html](https://hwkim3330.github.io/christmas-tree/mini.html) | 초경량 버전 | Firebase | ~4KB |

## 📦 7가지 버전

### 1️⃣ Standard (`index.html`)
풀 기능 버전으로, 모든 시각 효과와 Firebase 실시간 동기화를 지원합니다.
- 모든 방문자가 같은 트리를 함께 꾸밈
- 눈, 별, 달, 집 등 풍부한 배경
- 🎁 트리 밑 선물 상자들
- 🗑️ 오너먼트 개별 삭제 기능
- 📳 모바일 햅틱 피드백
- 사운드 효과 및 배경 음악 지원

### 2️⃣ WASM (`wasm.html`) ⚡ NEW
**WebAssembly 고성능 버전** - Rust로 구현된 파티클 시스템으로 최적화된 성능을 제공합니다.
- 🦀 **Rust + WebAssembly** 파티클 엔진
- ⚡ **300개 눈송이** (JS 버전 대비 2배)
- ✨ **WASM 기반 스파클 이펙트** (100개 동시 렌더링)
- 💡 **부드러운 조명 효과** (실시간 brightness 계산)
- 📊 **실시간 성능 통계** (FPS, 파티클 수)
- 🚀 **CPU 부하 감소** (네이티브 수준 성능)

```
wasm-snow/
├── Cargo.toml       # Rust 프로젝트 설정
└── src/lib.rs       # ParticleSystem, Sparkle, Light 구현
```

### 3️⃣ Lite (`lite.html`)
경량 버전으로, 저사양 기기나 빠른 로딩이 필요할 때 사용합니다.
- Firebase 실시간 동기화 (공유 기능)
- 심플한 디자인, 애니메이션 최소화
- 🗑️ 오너먼트 삭제 기능

### 4️⃣ Local (`local.html`)
**오프라인 개인용** 버전으로, 인터넷 연결 없이 사용할 수 있습니다.
- LocalStorage에 저장 (브라우저별 개별 저장)
- 데이터 내보내기/가져오기 (JSON)
- 🎁 트리 밑 선물 상자들
- 🗑️ 오너먼트 삭제 기능
- 인터넷 불필요, 완전 오프라인

### 5️⃣ Embed (`embed.html`)
웹사이트에 iframe으로 삽입 가능한 읽기 전용 버전입니다.
- 컨트롤 없이 트리만 표시
- 실시간 오너먼트 표시
- 눈, 조명 효과
- iframe 삽입에 최적화

```html
<iframe src="https://hwkim3330.github.io/christmas-tree/embed.html"
        width="400" height="500" frameborder="0"></iframe>
```

### 6️⃣ Kiosk (`kiosk.html`)
디지털 사이니지, 전시용 전체화면 버전입니다.
- 🖥️ 대형 디스플레이 최적화
- 📊 통계 표시 (총 오너먼트, 오늘 추가된 수)
- 📋 최근 추가된 오너먼트 목록
- ⏰ 실시간 시계
- 🖱️ 커서 숨김 (키오스크 모드)
- 더블클릭으로 전체화면

### 7️⃣ Mini (`mini.html`)
초경량 버전으로, 가장 작은 파일 크기입니다.
- ~4KB 극소 용량
- 핵심 기능만 포함 (추가, 이동)
- 저대역폭 환경에 적합

## ✨ Features

### 🎨 Visual Effects (Standard)
- 🌨️ **Canvas 눈 애니메이션** - 부드러운 눈 내리는 효과
- ⭐ **반짝이는 별** - 밤하늘 별들
- 🌙 **빛나는 달** - 따뜻한 달빛
- 🏠 **겨울 풍경** - 눈 덮인 집과 굴뚝 연기
- 💡 **트리 조명** - 다채로운 깜빡이는 조명
- ✨ **스파클 효과** - 오너먼트 추가/이동 시 반짝임
- 🎁 **선물 상자** - 트리 밑 리본 달린 선물들

### 🎱 Ornaments
- 🔴 **메탈릭 그라디언트** - 사실적인 크리스마스 볼
- 🏷️ **이름 표시** - 호버 시 전체 이름 툴팁
- ✋ **드래그 앤 드롭** - 자유로운 위치 이동
- 🎨 **자동 색상** - 이름 기반 고유 색상
- 🗑️ **삭제 기능** - 호버 시 X 버튼으로 삭제

### 🔄 Sync Options
- ☁️ **Firebase** (Standard/Lite/Embed/Kiosk/Mini) - 실시간 공유
- 💾 **LocalStorage** (Local) - 오프라인 저장
- 📤 **Export/Import** (Local) - JSON 백업/복원

### 🔊 Audio (Standard only)
- 🔔 **벨 사운드** - Web Audio API
- 🎵 **배경 음악** - song.mp3 지원
- 🔇 **음소거 토글**

### 📱 Mobile Support
- 📲 **터치 드래그** 지원
- 📳 **햅틱 피드백** - 진동 피드백
- 🎯 **반응형 디자인**
- 🔒 **Safe Area** (iPhone notch)

## 🚀 Quick Start

### GitHub Pages 배포

1. 이 저장소를 Fork 또는 Clone
2. GitHub Settings → Pages → Source: `main` branch
3. 몇 분 후 `https://[username].github.io/christmas-tree/` 에서 접속

### 로컬 실행

```bash
git clone https://github.com/hwkim3330/christmas-tree.git
cd christmas-tree

# 로컬 서버 (Python 3)
python3 -m http.server 8000

# 브라우저
open http://localhost:8000
```

> 💡 `local.html`은 서버 없이 파일을 직접 열어도 동작합니다!

## 🔧 Configuration

### Firebase 설정 (Standard/Lite)

`index.html` 또는 `lite.html`의 설정 수정:

```javascript
const CONFIG = {
    BASE_DB: "https://your-project.firebaseio.com",  // Firebase URL
    POLL_INTERVAL: 2000,      // 폴링 간격 (ms)
    FETCH_TIMEOUT: 10000,     // 타임아웃 (ms)
    MAX_RETRIES: 3,           // 재시도 횟수
    MAX_NAME_LENGTH: 12,      // 최대 이름 길이
    RATE_LIMIT_MS: 1000,      // 쿨다운 (ms)
};
```

### Firebase Database 규칙

```json
{
  "rules": {
    "balls": {
      ".read": true,
      ".write": true,
      "$ball_id": {
        ".validate": "newData.hasChildren(['name', 'x', 'y'])"
      }
    }
  }
}
```

### LocalStorage 설정 (Local)

`local.html`에서 저장 키 변경:

```javascript
const STORAGE_KEY = 'my_christmas_tree_v1';  // 변경 가능
```

## 📁 File Structure

```
christmas-tree/
├── index.html              # Standard 버전 (Firebase, 풀 기능)
├── wasm.html               # WASM 버전 (WebAssembly 고성능)
├── christmas_snow.js       # WASM JavaScript glue code
├── christmas_snow_bg.wasm  # WebAssembly 바이너리 (61KB)
├── lite.html               # Lite 버전 (Firebase, 경량)
├── local.html              # Local 버전 (LocalStorage, 오프라인)
├── embed.html              # Embed 버전 (iframe 삽입용)
├── kiosk.html              # Kiosk 버전 (디지털 사이니지용)
├── mini.html               # Mini 버전 (초경량)
├── song.mp3                # 배경 음악 (선택사항)
├── wasm-snow/              # Rust WASM 소스코드
│   ├── Cargo.toml
│   └── src/lib.rs
└── README.md
```

## 🎯 Version Comparison

| Feature | Standard | WASM | Lite | Local | Embed | Kiosk | Mini |
|---------|:--------:|:----:|:----:|:-----:|:-----:|:-----:|:----:|
| **Storage** | Firebase | Firebase | Firebase | LocalStorage | Firebase | Firebase | Firebase |
| **Sharing** | ✅ | ✅ | ✅ | ❌ | ✅ (읽기) | ✅ (읽기) | ✅ |
| **Offline** | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Add Ornament** | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ |
| **Delete Ornament** | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Drag & Drop** | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ |
| SVG Tree | 다층 | 다층 | 심플 | 다층 | 다층 | 다층 | 심플 |
| Snow | ✅ JS | ⚡ WASM | ❌ | ✅ JS | ✅ JS | ✅ JS | ❌ |
| Sparkles | ✅ JS | ⚡ WASM | ❌ | ❌ | ❌ | ❌ | ❌ |
| Tree Lights | ✅ CSS | ⚡ WASM | ❌ | ✅ CSS | ✅ CSS | ✅ CSS | ❌ |
| Stars | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ |
| Presents | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ❌ |
| Perf Stats | ❌ | ✅ FPS | ❌ | ❌ | ❌ | ❌ | ❌ |
| Haptic | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| Sound | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **File Size** | ~48KB | ~85KB | ~9KB | ~22KB | ~6KB | ~12KB | ~4KB |
| **Particles** | 150 | 300+ | - | 120 | 80 | 150 | - |

## 🛠️ Tech Stack

- **Frontend**: Pure HTML/CSS/JavaScript (No framework)
- **Database**: Firebase Realtime Database / LocalStorage
- **Graphics**: SVG, Canvas API
- **Audio**: Web Audio API
- **WASM**: Rust + wasm-bindgen + wasm-pack
- **Hosting**: GitHub Pages

### WASM 빌드 방법

```bash
# wasm-pack 설치
cargo install wasm-pack

# WASM 빌드
cd wasm-snow
wasm-pack build --target web --release

# 파일 복사
cp pkg/christmas_snow_bg.wasm ../
cp pkg/christmas_snow.js ../
```

## 📝 Data Structure

### Firebase (Standard/Lite/Embed/Kiosk/Mini)

```json
{
  "balls": {
    "-unique_id": {
      "name": "사용자 이름",
      "x": 50.5,
      "y": 65.3,
      "color": "#e74c3c",
      "createdAt": 1701388800000
    }
  }
}
```

### LocalStorage (Local)

```json
[
  {
    "name": "사용자 이름",
    "x": 50.5,
    "y": 65.3,
    "color": "#e74c3c",
    "time": 1701388800000
  }
]
```

## 🔒 Security Notes

- 입력 값 sanitization (`<>\"'&` 제거)
- XSS 방지 HTML escape
- Rate limiting (스팸 방지)
- 위치 값 검증 (0-100 범위)

## 🐛 Known Issues

- Safari Web Audio API 자동 재생 제한
- 구형 브라우저 CSS backdrop-filter 미지원
- LocalStorage 용량 제한 (~5MB)

## 📜 License

MIT License - 자유롭게 사용, 수정, 배포 가능

## 🤝 Contributing

1. Fork this repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open a Pull Request

---

**Merry Christmas! 🎄🎅🎁**

Made with ❤️ by [hwkim3330](https://github.com/hwkim3330)
