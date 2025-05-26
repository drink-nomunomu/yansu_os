package main

import (
	"log"
	"strings"
	"time"

	"github.com/atotto/clipboard"
)

// 行末 2 行を削除
func trimLastTwoLines(s string) string {
	// CRLF 対策で \r\n → \n へ統一
	s = strings.ReplaceAll(s, "\r\n", "\n")

	lines := strings.Split(s, "\n")
	if len(lines) <= 2 {
		return ""
	}
	return strings.Join(lines[:len(lines)-2], "\n")
}

func main() {
	var prev string // 前回処理した内容
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()

	for range ticker.C {
		current, err := clipboard.ReadAll()
		if err != nil {
			log.Printf("clipboard read error: %v", err)
			continue
		}

		// 前回と同じなら何もしない
		if current == prev {
			continue
		}

		// クリップボードの内容に「Kindle」が含まれている場合のみトリム処理を実行
		if !strings.Contains(current, "Kindle") {
			prev = current
			continue
		}

		trimmed := trimLastTwoLines(current)

		// 行末 2 行が存在しない場合や、すでに処理済みの場合はスキップ
		if trimmed == current || trimmed == prev {
			prev = current
			continue
		}

		if err := clipboard.WriteAll(trimmed); err != nil {
			log.Printf("clipboard write error: %v", err)
			continue
		}

		prev = trimmed
		log.Println("clipboard trimmed")
	}
}
