fn main() {
    println!("Hello, world!");
}


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

  
GỢI Ý KHÔNG HỢP LÝ.
TÊN SINH VIÊN VÀ ĐIỂM CỦA SINH VIÊN ĐÓ:
- 1 sinh viên sẽ có nhiều môn học, Cấu trúc phải thiết kế sao cho lưu được điểm của tất cả các môn của cùng 1 sinh viên.
- Lưu thế này sẽ duplicate tên của sinh viên rất nhiều lần, khi bổ sung 1 môn học mới.
- Các sinh viên ko có key unique để phân biệt. Nếu cùng tên cùng điểm số, thì ko biết ai là ai luôn.
- Do ko có key unique, giả sử nhập nhầm tên sinh viên, là cũng ko biết sửa chỗ nào cho đúng luôn.


cấu trúc hợp lý dự kiến như sau:

struct Student {
    name: String,
    student_id: String,
}

struct Subject {
    subject_name: String,
    score: String,
    student_id: String,
}

impl Student {
    pub fn new() -> Student {
        Student {
            name: String::new(),
            student_id: String::new(),
        }
    }

    pub fn add(&mut self, name: &str, student_id: &str) {
        self.name = name;
        self.student_id=student_id;
    }
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
        self.name = name;
        self.student_id=student_id;
        self.score=score;
    }
    
    // //2. Liệt kê các điểm số hiện tại mà trường đã cập nhật. Yêu cầu này chả có lý do gì phải trả về tên học sinh là gì cả. Tất cả điểm số unique là được
    
    pub fn grades(&self) -> Vec<u32> {
        unimplemented!()

    }
    
    //3. Liệt kê danh sách các học sinh có cùng 1 điểm số
    
    pub fn grade(&self, score: u32) -> Vec<String> {
        unimplemented!()
        
    }
}
