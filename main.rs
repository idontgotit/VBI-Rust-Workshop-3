
// Bài tập
// Cho ngữ cảnh như sau : Một ngôi trường cần lập danh
//sách thông tin sinh viên bao gồm tên sinh viên và điểm của sinh viên đó.
// với mục đích thống kê kiểm tra giáo dục của ngôi trường này

/*-----------------------------*/
// Gợi ý:
// Định nghĩa bằng struct, mọi người nên sử dụng HashMap 
// Tại sao lại sử dụng HashMap và ko phải Vec
//https://doc.rust-lang.org/std/collections/struct.HashMap.html
// struct School {
//     students: HashMap<String, u32>
// }

  
// GỢI Ý CẤU TRÚC struct School KHÔNG HỢP LÝ. <=======

// - 1 sinh viên sẽ có nhiều môn học, cấu trúc phải thiết kế sao cho lưu được điểm của tất cả các môn của cùng 1 sinh viên.
// - Lưu thế này sẽ duplicate tên của sinh viên rất nhiều lần, khi bổ sung 1 môn học mới.
// - Các sinh viên ko có key unique để phân biệt. Nếu cùng tên cùng điểm số, thì ko biết ai là ai luôn.
// - Do ko có key unique, giả sử người dùng nhập nhầm tên sinh viên, thì cũng ko biết sửa vào ai cho đúng.


// cấu trúc hợp lý dự kiến như sau:
// Subject là thành phần nhỏ nhất
// Student bao gồm nhiều subject
// School bao gồm nhiều student



use std::collections::HashMap;


struct Subject {
    subject_name: String,
    score: String,
    student_id: String,
}

struct Student {
    name: String,
    student_id: String,
    subjects: Vec<Subject>,
}

struct School {
    students: Vec<Student>,
}

impl Subject {
    pub fn new() -> Subject {
        Subject {
            subject_name: String::new(),
            student_id: String::new(),
            score: String::new(),
        }
    }

    pub fn add(&mut self, subject_name: &str, student_id: &str, score: &str) {
        self.subject_name = subject_name.to_string();
        self.student_id=student_id.to_string();
        self.score=score.to_string();
    }
}

impl Student {
    pub fn new() -> Student {
        Student {
            name: String::new(),
            student_id: String::new(),
            subjects: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &str, student_id: &str, subjects: Vec<Subject>) {
        self.name = name.to_string();
        self.student_id=student_id.to_string();
        self.subjects = subjects;
    }
}


impl School {
    pub fn new() -> School {
        School {
            students: Vec::new(),
        }
    }

    pub fn add(&mut self, students: Vec<Student>) {
        self.students = students;
        
    }

    // //2. Liệt kê các điểm số hiện tại mà trường đã cập nhật. Yêu cầu này chả có lý do gì phải trả về tên học sinh là gì cả. Tất cả điểm số unique là được
    
    pub fn scores(&self) -> Vec<String>{
        let mut result:Vec<String> = vec![];
        for student in self.students.iter() {
            
            for subject in student.subjects.iter() {
                result.push(subject.score.to_string())
            }
        }

        println!("{:?}", result);
        return result

    }
    
    //3. Liệt kê danh sách các học sinh có cùng 1 điểm số
    
    pub fn scores_with_names(&self, score: &str) -> Vec<String> {
        let mut result:Vec<String> = vec![];
        for student in self.students.iter() {
            for subject in student.subjects.iter() {
                if subject.score == score{
                    result.push(student.name.to_string())
                }
            }
        }

        println!("{:?}", result);
        return result
        
    }
}

fn main() {
    println!("Hello, VBI-Rust-Workshop-3!");

    let mut subject_math_of_alice: Subject= Subject::new();
    let mut subject_math_of_bob: Subject= Subject::new();
    let mut subject_math_of_charlie: Subject= Subject::new();
    subject_math_of_alice.add("Math", "MSSV_123", "3");
    subject_math_of_bob.add("Math", "MSSV_123", "10");
    subject_math_of_charlie.add("Math", "MSSV_123", "4");

    let mut student_alice: Student= Student::new();
    let mut student_bob: Student= Student::new();
    let mut student_charlie: Student= Student::new();
    student_alice.add("Alice", "MSSV_123", vec![subject_math_of_alice]);
    student_bob.add("Bob", "MSSV_456", vec![subject_math_of_bob]);
    student_charlie.add("Charlie", "MSSV_890", vec![subject_math_of_charlie]);


    let mut school_a: School= School::new();
    let mut vec_students = vec![student_alice, student_bob, student_charlie];
    school_a.add(vec_students);

    school_a.scores();

    school_a.scores_with_names("4");


}
