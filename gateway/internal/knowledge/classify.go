package knowledge

import (
	"encoding/json"
	"fmt"
	"strings"
)

type classifyRule struct {
	keywords []string // 命中任一关键词即匹配
	category string   // 分类路径，如 "学习/英语/词汇"
	tags     []string // 维度标签，如 ["subject:英语", "type:词汇"]
}

var classifyRules = []classifyRule{
	// ===== 英语子类（优先匹配，避免被"考研"等宽泛词吞掉） =====
	{
		keywords: []string{"词汇", "单词", "vocabulary", "词组", "英语单词", "高频词"},
		category: "学习/英语/词汇",
		tags:     []string{"subject:英语", "type:词汇"},
	},
	{
		keywords: []string{"语法", "grammar", "时态", "从句"},
		category: "学习/英语/语法",
		tags:     []string{"subject:英语", "type:语法"},
	},
	{
		keywords: []string{"听力", "listening"},
		category: "学习/英语/听力",
		tags:     []string{"subject:英语", "type:听力"},
	},
	{
		keywords: []string{"阅读", "reading", "阅读理解"},
		category: "学习/英语/阅读",
		tags:     []string{"subject:英语", "type:阅读"},
	},
	{
		keywords: []string{"作文", "写作", "writing", "essay", "范文"},
		category: "学习/英语/写作",
		tags:     []string{"subject:英语", "type:写作"},
	},
	{
		keywords: []string{"翻译", "translation", "英译中", "中译英"},
		category: "学习/英语/翻译",
		tags:     []string{"subject:英语", "type:翻译"},
	},
	{
		keywords: []string{"雅思", "托福", "ielts", "toefl", "gre", "gmat"},
		category: "学习/英语/留学考试",
		tags:     []string{"subject:英语", "type:留学考试"},
	},
	{
		keywords: []string{"四级", "六级", "cet4", "cet6", "英语四级", "英语六级", "大学英语"},
		category: "学习/英语/等级考试",
		tags:     []string{"subject:英语", "type:等级考试"},
	},

	// ===== 考研子类 =====
	{
		keywords: []string{"政治", "马原", "毛概", "思修", "史纲", "时政"},
		category: "学习/考研/政治",
		tags:     []string{"subject:政治", "type:考研"},
	},
	{
		keywords: []string{"考研数学", "数一", "数二", "数三"},
		category: "学习/考研/数学",
		tags:     []string{"subject:数学", "type:考研"},
	},
	{
		keywords: []string{"考研英语", "考研词汇"},
		category: "学习/考研/英语",
		tags:     []string{"subject:英语", "type:考研"},
	},

	// ===== 通用学习（考研放后面，让具体子类先匹配） =====
	{
		keywords: []string{"考研", "研究生", "硕士", "博士", "报考", "录取"},
		category: "学习/考研",
		tags:     []string{"subject:考研"},
	},
	{
		keywords: []string{"数学", "高数", "线代", "线性代数", "概率论", "微积分"},
		category: "学习/数学",
		tags:     []string{"subject:数学"},
	},
	{
		keywords: []string{"编程", "代码", "python", "java", "go语言", "golang", "rust", "c++", "javascript", "前端", "后端", "算法"},
		category: "学习/编程",
		tags:     []string{"subject:编程"},
	},
	{
		keywords: []string{"面试题", "八股文", "leetcode", "面经"},
		category: "学习/求职",
		tags:     []string{"type:求职"},
	},
	{
		keywords: []string{"教材", "课本", "教科书", "课件", "ppt", "讲义"},
		category: "学习/教材",
		tags:     []string{"type:教材"},
	},
	{
		keywords: []string{"试卷", "真题", "模拟题", "习题", "练习题"},
		category: "学习/试题",
		tags:     []string{"type:试题"},
	},
	{
		keywords: []string{"笔记", "知识点", "考点", "重点", "错题"},
		category: "学习/笔记",
		tags:     []string{"type:笔记"},
	},

	// ===== 工作类 =====
	{
		keywords: []string{"周报", "月报", "日报", "季度报", "年报", "工作总结", "汇报"},
		category: "工作/汇报",
		tags:     []string{"type:汇报"},
	},
	{
		keywords: []string{"合同", "协议", "条款", "甲方", "乙方", "签署"},
		category: "工作/合同",
		tags:     []string{"type:合同"},
	},
	{
		keywords: []string{"需求", "prd", "产品", "原型", "设计稿", "交互", "ui"},
		category: "工作/产品",
		tags:     []string{"type:产品"},
	},
	{
		keywords: []string{"技术方案", "架构", "设计文档", "技术文档", "接口文档", "api"},
		category: "工作/技术",
		tags:     []string{"type:技术"},
	},
	{
		keywords: []string{"会议", "纪要", "会议记录", "meeting", "讨论"},
		category: "工作/会议",
		tags:     []string{"type:会议"},
	},
	{
		keywords: []string{"简历", "resume", "cv", "求职"},
		category: "工作/求职",
		tags:     []string{"type:求职"},
	},
	{
		keywords: []string{"报销", "发票", "财务", "预算", "费用"},
		category: "工作/财务",
		tags:     []string{"type:财务"},
	},
	{
		keywords: []string{"kpi", "okr", "绩效", "目标", "考核"},
		category: "工作/管理",
		tags:     []string{"type:管理"},
	},

	// ===== 生活类 =====
	{
		keywords: []string{"菜谱", "食谱", "烹饪", "美食", "烘焙"},
		category: "生活/美食",
		tags:     []string{"type:教程"},
	},
	{
		keywords: []string{"健身", "运动", "跑步", "瑜伽", "减肥", "增肌"},
		category: "生活/健身",
		tags:     []string{"type:教程"},
	},
	{
		keywords: []string{"旅游", "旅行", "攻略", "游记", "景点"},
		category: "生活/旅行",
		tags:     []string{"type:攻略"},
	},
	{
		keywords: []string{"理财", "投资", "股票", "基金", "保险", "房产"},
		category: "生活/理财",
		tags:     []string{"subject:理财"},
	},
	{
		keywords: []string{"租房", "搬家", "买房", "装修"},
		category: "生活/居住",
		tags:     []string{},
	},

	// ===== 其他 =====
	{
		keywords: []string{"论文", "paper", "abstract", "introduction", "conclusion", "reference", "doi"},
		category: "学术/论文",
		tags:     []string{"type:论文"},
	},
	{
		keywords: []string{"专利", "发明", "知识产权"},
		category: "法律/专利",
		tags:     []string{"type:专利"},
	},
	{
		keywords: []string{"说明书", "manual", "使用说明", "操作手册"},
		category: "文档/手册",
		tags:     []string{"type:手册"},
	},
}

// classifyByRules 规则匹配分类，匹配成功返回 (category, tags, true)
func classifyByRules(title, content, filename string) (string, []string, bool) {
	text := strings.ToLower(title + " " + content + " " + filename)
	// 只取前 3000 字用于规则匹配，足够覆盖标题和开头
	if len(text) > 3000 {
		text = text[:3000]
	}

	for _, rule := range classifyRules {
		for _, kw := range rule.keywords {
			if strings.Contains(text, kw) {
				return rule.category, rule.tags, true
			}
		}
	}
	return "", nil, false
}

// classifyByLLM 用 LLM 推断分类，返回 (category, tags, error)
func (h *Handler) classifyByLLM(title, content string) (string, []string, error) {
	if h.settingsStore == nil || len(content) > 3000 {
		if len(content) > 3000 {
			content = content[:3000]
		} else {
			return "", nil, fmt.Errorf("settings not available")
		}
	}

	_, currentModel, _, _, _, _, _, _ := h.settingsStore.Snapshot()
	provider := FindProvider(h.cfg, currentModel.ProviderID)
	if provider.ProviderID == "" || provider.APIKey == "" {
		return "", nil, fmt.Errorf("LLM not configured")
	}

	prompt := `你是一个文档分类助手。根据文档标题和内容，输出 JSON 格式的分类结果。

输出格式（严格 JSON，不要解释）：
{"category": "分类路径（2-3级，如'学习/英语/词汇'）", "tags": ["key:value格式的标签", "如subject:英语", "type:教程"]}

分类路径参考：
- 学习/考研/政治、学习/英语/词汇、学习/编程、
- 工作/汇报、工作/合同、工作/技术
- 生活/美食、生活/旅行、学术/论文

标题：` + title + `
内容：` + content + `

请输出 JSON：`

	answer, err := sendChatCompletion(provider, currentModel.ModelID, prompt)
	if err != nil {
		return "", nil, err
	}

	// 解析 JSON 响应，容错处理
	answer = strings.TrimSpace(answer)
	if idx := strings.Index(answer, "{"); idx >= 0 {
		if end := strings.LastIndex(answer, "}"); end > idx {
			answer = answer[idx : end+1]
		}
	}

	var result struct {
		Category string   `json:"category"`
		Tags     []string `json:"tags"`
	}
	if err := json.Unmarshal([]byte(answer), &result); err != nil {
		return "", nil, fmt.Errorf("LLM 分类结果解析失败: %w", err)
	}
	return result.Category, result.Tags, nil
}

// classifyContent 混合分类：规则优先 → LLM 兜底 → 文件扩展名保底
func (h *Handler) classifyContent(title, content, filename string) (string, []string) {
	// 1. 规则匹配
	if cat, tags, ok := classifyByRules(title, content, filename); ok {
		return cat, tags
	}

	// 2. LLM 分类（异步友好的场景可以调用）
	if h.settingsStore != nil {
		if cat, tags, err := h.classifyByLLM(title, content); err == nil {
			return cat, tags
		}
	}

	// 3. 兜底：文件扩展名
	ext := "其他"
	if idx := strings.LastIndex(filename, "."); idx >= 0 {
		ext = strings.ToLower(filename[idx+1:])
	}
	return "文档/" + ext, []string{"source:upload", "type:" + ext}
}
