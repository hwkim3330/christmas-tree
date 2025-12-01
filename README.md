# 🎄 Shared Christmas Tree

> 친구, 가족과 함께 크리스마스 트리를 꾸며보세요!

실시간으로 동기화되는 공유 크리스마스 트리 웹앱입니다. 트리를 클릭하고 이름을 입력하면 오너먼트(장식)가 추가됩니다. 모든 방문자가 같은 트리를 함께 꾸밀 수 있습니다.

## 🌐 Live Demo

**[https://hwkim3330.github.io/christmas-tree/](https://hwkim3330.github.io/christmas-tree/)**

| Version | URL | Description |
|---------|-----|-------------|
| **Standard** | [index.html](https://hwkim3330.github.io/christmas-tree/) | 풀 기능 버전 (눈, 별, 집, 조명 애니메이션) |
| **Lite** | [lite.html](https://hwkim3330.github.io/christmas-tree/lite.html) | 경량 버전 (빠른 로딩, 저사양 기기용) |

## ✨ Features

### 🎨 Visual Effects
- 🌨️ **Canvas 기반 눈 애니메이션** - 부드러운 눈 내리는 효과
- ⭐ **반짝이는 별** - 밤하늘을 수놓는 별들
- 🌙 **빛나는 달** - 따뜻한 달빛
- 🏠 **겨울 풍경** - 눈 덮인 집과 굴뚝 연기
- 💡 **트리 조명** - 다채로운 색상의 깜빡이는 조명
- ✨ **스파클 효과** - 오너먼트 추가/이동 시 반짝임

### 🎱 Ornaments
- 🔴 **메탈릭 그라디언트** - 사실적인 크리스마스 볼 디자인
- 🏷️ **이름 표시** - 마우스 호버 시 전체 이름 툴팁
- ✋ **드래그 앤 드롭** - 오너먼트 위치 자유롭게 이동
- 🎨 **자동 색상** - 이름에 따라 고유 색상 자동 할당

### 🔄 Real-time Sync
- ☁️ **Firebase Realtime Database** - 실시간 데이터 동기화
- 📡 **2초 폴링** - 다른 사용자의 변경사항 자동 반영
- 🔌 **오프라인 감지** - 연결 상태 표시
- ⏱️ **Rate Limiting** - 스팸 방지 (1초 쿨다운)

### 🔊 Audio
- 🔔 **벨 사운드** - Web Audio API 기반 효과음
- 🎵 **배경 음악** - song.mp3 파일 지원 (선택사항)
- 🔇 **음소거 토글** - 사운드 ON/OFF

### 📱 Mobile Support
- 📲 **터치 지원** - 모바일에서 드래그 가능
- 🎯 **반응형 디자인** - 모든 화면 크기 대응
- 🔒 **Safe Area** - 아이폰 노치/홈바 대응

## 🚀 Quick Start

### GitHub Pages 배포

1. 이 저장소를 Fork 또는 Clone
2. GitHub Settings → Pages → Source: `main` branch
3. 몇 분 후 `https://[username].github.io/christmas-tree/` 에서 접속

### 로컬 실행

```bash
# Clone
git clone https://github.com/hwkim3330/christmas-tree.git
cd christmas-tree

# 로컬 서버 실행 (Python 3)
python3 -m http.server 8000

# 브라우저에서 열기
open http://localhost:8000
```

## 🔧 Configuration

### Firebase 설정 변경

`index.html`의 CONFIG 객체를 수정하세요:

```javascript
const CONFIG = {
    BASE_DB: "https://your-project.firebaseio.com",  // 여기 변경
    POLL_INTERVAL: 2000,      // 폴링 간격 (ms)
    FETCH_TIMEOUT: 10000,     // 요청 타임아웃 (ms)
    MAX_RETRIES: 3,           // 재시도 횟수
    MAX_NAME_LENGTH: 12,      // 최대 이름 길이
    RATE_LIMIT_MS: 1000,      // 추가 쿨다운 (ms)
};
```

### Firebase Realtime Database 규칙

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

### 배경 음악 추가

`song.mp3` 파일을 같은 폴더에 추가하면 음악 버튼이 활성화됩니다.

## 📁 File Structure

```
christmas-tree/
├── index.html      # 메인 버전 (풀 기능)
├── lite.html       # 경량 버전 (빠른 로딩)
├── song.mp3        # 배경 음악 (선택사항)
└── README.md       # 이 파일
```

## 🎯 Version Comparison

| Feature | Standard | Lite |
|---------|:--------:|:----:|
| SVG Tree | ✅ 다층 그라디언트 | ✅ 심플 |
| Snow Animation | ✅ Canvas | ❌ |
| Stars | ✅ | ❌ |
| House & Moon | ✅ | ❌ |
| Tree Lights | ✅ | ❌ |
| Sparkle Effects | ✅ | ❌ |
| Metallic Ornaments | ✅ | ✅ 심플 |
| Drag & Drop | ✅ | ✅ |
| Firebase Sync | ✅ | ✅ |
| Sound Effects | ✅ | ❌ |
| Background Music | ✅ | ❌ |
| Error Handling | ✅ 강화 | ✅ 기본 |
| File Size | ~45KB | ~8KB |
| Load Time | ~1s | ~0.3s |

## 🛠️ Tech Stack

- **Frontend**: Pure HTML/CSS/JavaScript (No framework)
- **Database**: Firebase Realtime Database
- **Graphics**: SVG, Canvas API
- **Audio**: Web Audio API
- **Hosting**: GitHub Pages

## 📝 Data Structure

Firebase에 저장되는 오너먼트 데이터:

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

## 🔒 Security Notes

- 입력 값 sanitization 적용 (`<`, `>`, `"`, `'`, `&` 제거)
- XSS 방지를 위한 HTML escape
- Rate limiting으로 스팸 방지
- 위치 값 검증 (0-100 범위)

## 🐛 Known Issues

- Safari에서 Web Audio API 자동 재생 제한 (사용자 인터랙션 필요)
- 일부 구형 브라우저에서 CSS backdrop-filter 미지원

## 📜 License

MIT License - 자유롭게 사용, 수정, 배포 가능합니다.

## 🤝 Contributing

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

---

**Merry Christmas! 🎄🎅🎁**

Made with ❤️ by [hwkim3330](https://github.com/hwkim3330)
