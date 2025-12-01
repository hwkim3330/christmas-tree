# 🎄 Shared Christmas Tree

> 친구, 가족과 함께 크리스마스 트리를 꾸며보세요!

실시간으로 동기화되는 공유 크리스마스 트리 웹앱입니다. 트리를 클릭하고 이름을 입력하면 오너먼트(장식)가 추가됩니다.

## 🌐 Live Demo

**[https://hwkim3330.github.io/christmas-tree/](https://hwkim3330.github.io/christmas-tree/)**

### 버전 선택

| Version | URL | Description | Storage |
|---------|-----|-------------|---------|
| **Standard** | [index.html](https://hwkim3330.github.io/christmas-tree/) | 풀 기능 버전 (눈, 별, 집, 조명) | Firebase |
| **Lite** | [lite.html](https://hwkim3330.github.io/christmas-tree/lite.html) | 경량 버전 (빠른 로딩) | Firebase |
| **Local** | [local.html](https://hwkim3330.github.io/christmas-tree/local.html) | 오프라인 개인용 | LocalStorage |

## 📦 3가지 버전

### 1️⃣ Standard (`index.html`)
풀 기능 버전으로, 모든 시각 효과와 Firebase 실시간 동기화를 지원합니다.
- 모든 방문자가 같은 트리를 함께 꾸밈
- 눈, 별, 달, 집 등 풍부한 배경
- 사운드 효과 및 배경 음악 지원
- ~45KB, 로딩 ~1초

### 2️⃣ Lite (`lite.html`)
경량 버전으로, 저사양 기기나 빠른 로딩이 필요할 때 사용합니다.
- Firebase 실시간 동기화 (공유 기능)
- 심플한 디자인, 애니메이션 최소화
- ~8KB, 로딩 ~0.3초

### 3️⃣ Local (`local.html`)
**오프라인 개인용** 버전으로, 인터넷 연결 없이 사용할 수 있습니다.
- LocalStorage에 저장 (브라우저별 개별 저장)
- 데이터 내보내기/가져오기 (JSON)
- 인터넷 불필요, 완전 오프라인
- ~20KB

## ✨ Features

### 🎨 Visual Effects (Standard)
- 🌨️ **Canvas 눈 애니메이션** - 부드러운 눈 내리는 효과
- ⭐ **반짝이는 별** - 밤하늘 별들
- 🌙 **빛나는 달** - 따뜻한 달빛
- 🏠 **겨울 풍경** - 눈 덮인 집과 굴뚝 연기
- 💡 **트리 조명** - 다채로운 깜빡이는 조명
- ✨ **스파클 효과** - 오너먼트 추가/이동 시 반짝임

### 🎱 Ornaments
- 🔴 **메탈릭 그라디언트** - 사실적인 크리스마스 볼
- 🏷️ **이름 표시** - 호버 시 전체 이름 툴팁
- ✋ **드래그 앤 드롭** - 자유로운 위치 이동
- 🎨 **자동 색상** - 이름 기반 고유 색상

### 🔄 Sync Options
- ☁️ **Firebase** (Standard/Lite) - 실시간 공유
- 💾 **LocalStorage** (Local) - 오프라인 저장
- 📤 **Export/Import** (Local) - JSON 백업/복원

### 🔊 Audio (Standard only)
- 🔔 **벨 사운드** - Web Audio API
- 🎵 **배경 음악** - song.mp3 지원
- 🔇 **음소거 토글**

### 📱 Mobile Support
- 📲 **터치 드래그** 지원
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
├── index.html      # Standard 버전 (Firebase, 풀 기능)
├── lite.html       # Lite 버전 (Firebase, 경량)
├── local.html      # Local 버전 (LocalStorage, 오프라인)
├── song.mp3        # 배경 음악 (선택사항)
└── README.md
```

## 🎯 Version Comparison

| Feature | Standard | Lite | Local |
|---------|:--------:|:----:|:-----:|
| **Storage** | Firebase | Firebase | LocalStorage |
| **Sharing** | ✅ 실시간 공유 | ✅ 실시간 공유 | ❌ 개인용 |
| **Offline** | ❌ | ❌ | ✅ 완전 오프라인 |
| SVG Tree | 다층 그라디언트 | 심플 | 다층 그라디언트 |
| Snow | ✅ Canvas | ❌ | ✅ Canvas |
| Stars | ✅ | ❌ | ✅ |
| House & Moon | ✅ | ❌ | ✅ Moon only |
| Tree Lights | ✅ | ❌ | ✅ |
| Sparkle Effects | ✅ | ❌ | ❌ |
| Sound Effects | ✅ | ❌ | ❌ |
| Background Music | ✅ | ❌ | ❌ |
| Export/Import | ❌ | ❌ | ✅ JSON |
| Error Handling | 강화 | 기본 | 기본 |
| **File Size** | ~45KB | ~8KB | ~20KB |
| **Load Time** | ~1s | ~0.3s | ~0.5s |

## 🛠️ Tech Stack

- **Frontend**: Pure HTML/CSS/JavaScript (No framework)
- **Database**: Firebase Realtime Database / LocalStorage
- **Graphics**: SVG, Canvas API
- **Audio**: Web Audio API
- **Hosting**: GitHub Pages

## 📝 Data Structure

### Firebase (Standard/Lite)

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
