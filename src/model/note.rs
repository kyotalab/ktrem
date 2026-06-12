use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Zettel {
    pub id: String,        // "1/1/2"（表示ID）
    pub file_name: String, // "1-1-2"（ファイル名、拡張子なし）
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub content: String, // 本文全体
}

impl Zettel {
    pub fn title(&self) -> String {
        // contentからH1を抽出して返す
        extract_h1(&self.content).unwrap_or_else(|| "Untitled".to_string())
    }

    pub fn id_from_file_name(file_name: &str) -> String {
        file_name.replace('-', "/")
    }

    /// 指定したidの祖先かどうか判定
    pub fn is_ancestor_of(&self, other_id: &str) -> bool {
        other_id != self.id && other_id.starts_with(&self.id)
    }

    /// 本文中の [[...]] 形式のリンクを抽出してID一覧を返す
    pub fn extract_links(&self) -> Vec<String> {
        let mut links = Vec::new();
        let mut chars = self.content.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '[' && chars.peek() == Some(&'[') {
                chars.next(); // 2つ目の '[' を消費

                let mut id = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ']' {
                        break;
                    }
                    id.push(c);
                    chars.next();
                }

                // "]]" を消費
                if chars.peek() == Some(&']') {
                    chars.next();
                    if chars.peek() == Some(&']') {
                        chars.next();
                        links.push(id);
                    }
                }
            }
        }

        links
    }
}

#[derive(Clone)]
pub struct Scratch {
    pub timestamp: String, // "202506061430"
    pub content: String,   // 本文全体
}

impl Scratch {
    pub fn preview(&self) -> String {
        // H1があればH1、なければ本文冒頭を返す
        extract_h1(&self.content).unwrap_or_else(|| {
            // H1がなければ本文冒頭を返す
            self.content
                .lines()
                .find(|line| !line.trim().is_empty())
                .unwrap_or("(empty)")
                .to_string()
        })
    }
}

// H1を取り出す共通ロジック
fn extract_h1(content: &str) -> Option<String> {
    content
        .lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches("# ").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links_single() {
        let zettel = Zettel {
            id: "1".to_string(),
            file_name: "1".to_string(),
            tags: vec![],
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            content: "See [[1a]] for details.".to_string(),
        };
        assert_eq!(zettel.extract_links(), vec!["1a".to_string()]);
    }

    #[test]
    fn test_extract_links_multiple() {
        let zettel = Zettel {
            id: "1".to_string(),
            file_name: "1".to_string(),
            tags: vec![],
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            content: "Related: [[1a]] and [[1/1]].".to_string(),
        };
        assert_eq!(
            zettel.extract_links(),
            vec!["1a".to_string(), "1/1".to_string()]
        );
    }

    #[test]
    fn test_extract_links_none() {
        let zettel = Zettel {
            id: "1".to_string(),
            file_name: "1".to_string(),
            tags: vec![],
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            content: "No links here.".to_string(),
        };
        assert_eq!(zettel.extract_links(), Vec::<String>::new());
    }
}
