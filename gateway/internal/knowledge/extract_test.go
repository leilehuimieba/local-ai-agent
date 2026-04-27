package knowledge

import "testing"

func TestIsMostlyGarbledWithC1Controls(t *testing.T) {
	// 构造 C1 控制字符占主导的乱码文本，确保排除后 validRatio < 30%
	var b []rune
	for i := 0; i < 1000; i++ {
		b = append(b, '\u0080', '\u0092', '\u0093', '\u0094', '\u0095', '\u0096', '\u0097', '\u0098')
		b = append(b, '!')
	}
	if !isMostlyGarbled(string(b)) {
		t.Fatalf("isMostlyGarbled should detect C1-heavy garbled text")
	}
}

func TestIsMostlyGarbledNormalChinese(t *testing.T) {
	s := "第一章大学英语四级高频词汇\nstir搅拌；使微动；打动，产生\n"
	if isMostlyGarbled(s) {
		t.Fatalf("isMostlyGarbled should not flag normal Chinese text")
	}
}
