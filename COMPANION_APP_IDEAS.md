# Companion App Ideas

A mobile companion app connecting via Bluetooth to the chess board.

## Core Features

### Chess Engine & Analysis
- Run Stockfish natively on the phone
- Real-time position evaluation bar
- Best move suggestions with arrows/highlights
- Post-game analysis with accuracy scores
- Blunder/mistake/inaccuracy detection

### Online Play
- Chess.com / Lichess API integration
- Play online games reflected on the physical board
- Challenge friends, play bots, enter tournaments
- Sync ratings and game history

### Game Management
| Feature | Description |
|---------|-------------|
| Game database | Store all games with cloud backup |
| PGN import/export | Share games, import famous games to replay |
| Position setup | Set up puzzles or custom positions |
| Game resume | Save and continue later |

## Learning & Training

### Opening Explorer
- Opening book with millions of positions
- Show popular continuations and win rates
- Name the opening being played
- Suggest repertoire moves

### Tactics Training
- Daily puzzles pushed to the board
- Themed training (pins, forks, back rank, etc.)
- Track puzzle rating over time

### Endgame Tablebases
- Perfect play in 6-7 piece endgames
- Show "Mate in X" or "Draw"

## User Experience

### Visual Board Mirror
- 2D/3D board view synced with physical board
- Move arrows and highlights
- Captured pieces display
- Material advantage indicator

### Audio & Voice
- Move announcements ("Knight to f3")
- Voice control ("Show me the best move")
- Commentary mode for analysis

### Clock & Timer
- Full-featured chess clock UI
- Multiple time controls (bullet, blitz, rapid, classical)
- Increment/delay options
- Low time warnings sent to board buzzer

## Social & Stats

### Player Profiles
- Track Elo progression over time
- Win/loss/draw statistics
- Opening performance stats
- Time-of-day performance trends

### Multiplayer
- Local multiplayer via Bluetooth (two phones, one board)
- Remote play with friends
- Spectator mode

### Achievements & Gamification
- Badges for milestones
- Daily streaks
- Rating goals

## Settings & Configuration

- LED brightness/colors
- Sound preferences
- Assistance level (how many hints)
- Board orientation
- Piece detection calibration

## Bluetooth Protocol

Simple command protocol between app and board:

```
Board → App: MOVE e2e4      (piece moved)
Board → App: POSITION rnbq... (full FEN)
App → Board: LED e4 green   (light up square)
App → Board: SOUND capture  (play sound)
App → Board: CLOCK 05:00 04:32 (update times)
```

The app handles all the "smart" features while the board reports moves and displays feedback.
