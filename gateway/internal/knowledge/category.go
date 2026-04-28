package knowledge

import (
	"sort"
	"strings"
)

// ParseCategoryPath 将 "技术/考研/英语" 拆分为 ["技术", "考研", "英语"]
func ParseCategoryPath(category string) []string {
	if category == "" {
		return nil
	}
	parts := strings.Split(category, "/")
	var result []string
	for _, p := range parts {
		p = strings.TrimSpace(p)
		if p != "" {
			result = append(result, p)
		}
	}
	return result
}

// BuildCategoryTree 从扁平的分类名列表构建分类树
func BuildCategoryTree(categories []string) []CategoryNode {
	root := make(map[string]*CategoryNode)

	for _, cat := range categories {
		parts := ParseCategoryPath(cat)
		if len(parts) == 0 {
			continue
		}

		// 第一级
		first := parts[0]
		if root[first] == nil {
			root[first] = &CategoryNode{Name: first, Path: first}
		}
		root[first].Count++

		// 子级
		parent := root[first]
		path := first
		for i := 1; i < len(parts); i++ {
			path += "/" + parts[i]
			var child *CategoryNode
			for j := range parent.Children {
				if parent.Children[j].Name == parts[i] {
					child = &parent.Children[j]
					break
				}
			}
			if child == nil {
				parent.Children = append(parent.Children, CategoryNode{Name: parts[i], Path: path})
				child = &parent.Children[len(parent.Children)-1]
			}
			child.Count++
			parent = child
		}
	}

	result := make([]CategoryNode, 0, len(root))
	for _, node := range root {
		result = append(result, *node)
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Name < result[j].Name })
	for i := range result {
		sortCategoryChildren(&result[i])
	}
	return result
}

func sortCategoryChildren(node *CategoryNode) {
	if len(node.Children) == 0 {
		return
	}
	sort.Slice(node.Children, func(i, j int) bool { return node.Children[i].Name < node.Children[j].Name })
	for i := range node.Children {
		sortCategoryChildren(&node.Children[i])
	}
}

// ParseTagKV 解析 Key:Value 标签
// "type:PDF" → ("type", "PDF")
// "考研" → ("", "考研")
func ParseTagKV(tag string) (key, value string) {
	idx := strings.Index(tag, ":")
	if idx <= 0 {
		return "", tag
	}
	return tag[:idx], tag[idx+1:]
}

// GetTagDimensions 将标签按维度分组
// ["type:PDF", "year:2025", "考研"] → {"type": ["PDF"], "year": ["2025"], "": ["考研"]}
func GetTagDimensions(tags []string) map[string][]string {
	dims := make(map[string][]string)
	for _, t := range tags {
		k, v := ParseTagKV(t)
		dims[k] = append(dims[k], v)
	}
	return dims
}

// FilterByTagDimension 过滤出含有指定 key:value 标签的 items
func FilterByTagDimension(items []Item, key, value string) []Item {
	var result []Item
	for _, it := range items {
		for _, t := range it.Tags {
			k, v := ParseTagKV(t)
			if k == key && (value == "" || v == value) {
				result = append(result, it)
				break
			}
		}
	}
	return result
}

// CategoryCounts 统计每个分类下的条目数（用于构建列表页面的 counts）
func CategoryCounts(categories []string) map[string]int {
	counts := make(map[string]int)
	for _, cat := range categories {
		for _, part := range ParseCategoryPath(cat) {
			counts[part]++
		}
	}
	return counts
}

// FlattenCategories 展开所有分类路径（支持用 LIKE 前缀匹配子分类）
func FlattenCategories(categories []string) []string {
	seen := make(map[string]bool)
	var result []string
	for _, cat := range categories {
		parts := ParseCategoryPath(cat)
		path := ""
		for _, p := range parts {
			if path != "" {
				path += "/"
			}
			path += p
			if !seen[path] {
				seen[path] = true
				result = append(result, path)
			}
		}
	}
	sort.Strings(result)
	return result
}
