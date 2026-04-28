package knowledge

import "testing"

func TestClassifyByRules_Hit(t *testing.T) {
	tests := []struct {
		title, content, filename string
		wantCategory             string
		wantTag                  string
	}{
		{"考研英语词汇", "stir 搅拌", "词汇.pdf", "学习/英语/词汇", "subject:英语"},
		{"四级真题", "大学英语四级考试", "真题.docx", "学习/英语/等级考试", "type:等级考试"},
		{"托福听力", "TOEFL listening", "听力.pdf", "学习/英语/听力", "type:听力"},
		{"政治笔记", "马原重点归纳", "笔记.txt", "学习/考研/政治", "subject:政治"},
		{"周报", "本周工作内容", "周报.docx", "工作/汇报", "type:汇报"},
		{"合同", "甲方乙方签署协议", "合同.pdf", "工作/合同", "type:合同"},
		{"Python入门", "编程语言教程", "python.md", "学习/编程", "subject:编程"},
		{"健身计划", "减肥增肌", "健身.md", "生活/健身", "type:教程"},
		{"学术论文", "abstract introduction", "paper.pdf", "学术/论文", "type:论文"},
	}

	for _, tt := range tests {
		cat, tags, ok := classifyByRules(tt.title, tt.content, tt.filename)
		if !ok {
			t.Errorf("classifyByRules(%q) should match", tt.title)
			continue
		}
		if cat != tt.wantCategory {
			t.Errorf("classifyByRules(%q) category = %q, want %q", tt.title, cat, tt.wantCategory)
		}
		found := false
		for _, tag := range tags {
			if tag == tt.wantTag {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("classifyByRules(%q) tags = %v, want containing %q", tt.title, tags, tt.wantTag)
		}
	}
}

func TestClassifyByRules_Miss(t *testing.T) {
	_, _, ok := classifyByRules("未知文档", "无关键词内容", "data.bin")
	if ok {
		t.Errorf("should not match unknown content")
	}
}

func TestClassifyByRules_EmptyContent(t *testing.T) {
	_, _, ok := classifyByRules("四级", "", "")
	if !ok {
		t.Errorf("title '四级' should match from title alone")
	}
}

func TestClassifyByRules_FilenameMatch(t *testing.T) {
	_, _, ok := classifyByRules("", "", "英语四级真题.pdf")
	if !ok {
		t.Errorf("filename containing '四级' should match")
	}
}
