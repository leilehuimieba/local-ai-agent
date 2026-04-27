package knowledge

import "testing"

func TestCleanWatermark(t *testing.T) {
	input := "第一章 词汇\n淘宝店铺：光速考研工作室\n第二章 阅读\n掌柜旺旺：xxx\n第三章 写作\n"
	got := cleanWatermark(input)
	want := "第一章 词汇\n第二章 阅读\n第三章 写作"
	if got != want {
		t.Fatalf("cleanWatermark mismatch:\ngot:\n%s\nwant:\n%s", got, want)
	}
}

func TestCleanWatermarkCompressEmptyLines(t *testing.T) {
	input := "a\n\n\n淘宝：foo\n\n\nb"
	got := cleanWatermark(input)
	want := "a\n\nb"
	if got != want {
		t.Fatalf("cleanWatermark should compress consecutive empty lines, got:\n%s", got)
	}
}

func TestCleanWatermarkNoMatch(t *testing.T) {
	input := "正常中文内容\n没有任何水印\n"
	got := cleanWatermark(input)
	want := "正常中文内容\n没有任何水印"
	if got != want {
		t.Fatalf("cleanWatermark should not alter clean text, got:\n%s", got)
	}
}

func TestHasWatermarkLine(t *testing.T) {
	cases := []struct {
		line string
		want bool
	}{
		{"淘宝店铺：foo", true},
		{"认准淘宝店铺：bar", true},
		{"叮当考研 资料", true},
		{"谈辰图书", true},
		{"学海无涯教育", true},
		{"光速考研工作室", true},
		{"掌柜旺旺：abc", true},
		{"淘宝：123", true},
		{"正常学习内容", false},
		{"", false},
	}
	for _, c := range cases {
		if got := hasWatermarkLine(c.line); got != c.want {
			t.Fatalf("hasWatermarkLine(%q) = %v, want %v", c.line, got, c.want)
		}
	}
}
