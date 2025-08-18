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

// EPUBやその他の著作権表示を検出・削除する関数
func removeEpubCopyright(text string) (string, bool) {
	// CRLF 対策で \r\n → \n へ統一
	text = strings.ReplaceAll(text, "\r\n", "\n")
	
	// 著作権表示のパターン
	patterns := []string{
		"Excerpt From",
		"This material may be protected by copyright.",
		"This material may be protected by copyright",
		"抜粋:",
		"この作品は著作権で保護されている可能性があります。",
		"この作品は著作権で保護されている可能性があります",
	}
	
	for _, pattern := range patterns {
		if idx := strings.Index(text, pattern); idx != -1 {
			// パターンが見つかった位置以前のテキストを取得
			trimmed := strings.TrimSpace(text[:idx])
			return trimmed, true
		}
	}
	
	return text, false
}

// テキストが処理対象かどうかを判定する関数
func shouldProcess(text string) bool {
	// Kindleの場合
	if strings.Contains(text, "Kindle") {
		return true
	}
	
	// EPUBの著作権表示パターンをチェック
	epubPatterns := []string{
		"Excerpt From",
		"This material may be protected by copyright",
		"抜粋:",
		"この作品は著作権で保護されている可能性があります",
	}
	
	for _, pattern := range epubPatterns {
		if strings.Contains(text, pattern) {
			return true
		}
	}
	
	return false
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

		// 処理対象かどうかを判定
		if !shouldProcess(current) {
			prev = current
			continue
		}

		var trimmed string
		var processed bool

		// まずEPUBの著作権表示を試す
		if epubTrimmed, isEpub := removeEpubCopyright(current); isEpub {
			trimmed = epubTrimmed
			processed = true
			log.Println("EPUB copyright removed")
		} else if strings.Contains(current, "Kindle") {
			// Kindleの場合は従来の末尾2行削除
			trimmed = trimLastTwoLines(current)
			processed = true
			log.Println("Kindle last 2 lines trimmed")
		}

		// 処理されなかった場合や、変更がない場合はスキップ
		if !processed || trimmed == current || trimmed == prev {
			prev = current
			continue
		}

		if err := clipboard.WriteAll(trimmed); err != nil {
			log.Printf("clipboard write error: %v", err)
			continue
		}

		prev = trimmed
	}
}
