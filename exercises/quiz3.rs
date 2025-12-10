// quiz3.rs
// 定义通用的 Grade trait，所有支持的成绩类型都需实现该 trait
pub trait GradeDisplay {
    fn to_grade_string(&self) -> String;
}

// 为 f32 实现 GradeDisplay（数值型成绩）
impl GradeDisplay for f32 {
    fn to_grade_string(&self) -> String {
        self.to_string()
    }
}

// 为 &str 实现 GradeDisplay（字母型成绩，临时字符串）
impl GradeDisplay for &str {
    fn to_grade_string(&self) -> String {
        self.to_string()
    }
}

// 为 String 实现 GradeDisplay（字母型成绩，所有权字符串）
impl GradeDisplay for String {
    fn to_grade_string(&self) -> String {
        self.clone()
    }
}

// 改造 ReportCard 为泛型结构体，约束 T 实现 GradeDisplay
pub struct ReportCard<T: GradeDisplay> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: GradeDisplay> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.grade.to_grade_string() // 调用通用的格式化方法
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1, // f32 类型，自动适配
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+", // 改为字母型成绩（&str 类型）
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}