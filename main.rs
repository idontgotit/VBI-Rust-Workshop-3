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


