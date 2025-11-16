use crate::models::Language;

pub struct Translations {
    pub lang: Language,
}

impl Translations {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }

    pub fn heading(&self) -> &'static str {
        match self.lang {
            Language::English => "The Market Vietnamese - Korean Supermarket",
            Language::Vietnamese => "The Market Siêu thị việt hàn",
        }
    }

    pub fn search(&self) -> &'static str {
        match self.lang {
            Language::English => "Search:",
            Language::Vietnamese => "Tìm kiếm:",
        }
    }

    pub fn available_items(&self) -> &'static str {
        match self.lang {
            Language::English => "Available Items",
            Language::Vietnamese => "Mặt hàng có sẵn",
        }
    }

    pub fn selected_items(&self) -> &'static str {
        match self.lang {
            Language::English => "Selected Items",
            Language::Vietnamese => "Mặt hàng đã chọn",
        }
    }

    pub fn no_items_selected(&self) -> &'static str {
        match self.lang {
            Language::English => "No items selected",
            Language::Vietnamese => "Chưa chọn mặt hàng",
        }
    }

    pub fn total_cost(&self) -> &'static str {
        match self.lang {
            Language::English => "Total Cost:",
            Language::Vietnamese => "Tổng chi phí:",
        }
    }

    pub fn print_list(&self) -> &'static str {
        match self.lang {
            Language::English => "Print List",
            Language::Vietnamese => "In danh sách",
        }
    }

    pub fn printed_list(&self) -> &'static str {
        match self.lang {
            Language::English => "Printed List:",
            Language::Vietnamese => "Danh sách in:",
        }
    }

    pub fn copy_to_clipboard(&self) -> &'static str {
        match self.lang {
            Language::English => "Copy to Clipboard",
            Language::Vietnamese => "Sao chép vào bộ nhớ đệm",
        }
    }

    pub fn phan(&self) -> &'static str {
        match self.lang {
            Language::English => "items",
            Language::Vietnamese => "phần",
        }
    }

    pub fn won(&self) -> &'static str {
        match self.lang {
            Language::English => "won",
            Language::Vietnamese => "won",
        }
    }

    pub fn english(&self) -> &'static str {
        "English"
    }

    pub fn vietnamese(&self) -> &'static str {
        "Tiếng Việt"
    }

    pub fn edit_list(&self) -> &'static str {
        match self.lang {
            Language::English => "Edit Raw List",
            Language::Vietnamese => "Chỉnh sửa danh sách thô",
        }
    }

    pub fn save(&self) -> &'static str {
        match self.lang {
            Language::English => "Save",
            Language::Vietnamese => "Lưu",
        }
    }

    pub fn cancel(&self) -> &'static str {
        match self.lang {
            Language::English => "Cancel",
            Language::Vietnamese => "Hủy",
        }
    }

    pub fn raw_list_editor(&self) -> &'static str {
        match self.lang {
            Language::English => "Raw List Editor",
            Language::Vietnamese => "Trình chỉnh sửa danh sách thô",
        }
    }

    pub fn all_items(&self) -> &'static str {
        match self.lang {
            Language::English => "All Items",
            Language::Vietnamese => "Tất cả mặt hàng",
        }
    }

    pub fn item_manager(&self) -> &'static str {
        match self.lang {
            Language::English => "Item Manager",
            Language::Vietnamese => "Quản lý mặt hàng",
        }
    }

    pub fn add_item(&self) -> &'static str {
        match self.lang {
            Language::English => "Add Item",
            Language::Vietnamese => "Thêm mặt hàng",
        }
    }

    pub fn item_code(&self) -> &'static str {
        match self.lang {
            Language::English => "Item Code",
            Language::Vietnamese => "Mã mặt hàng",
        }
    }

    pub fn item_description(&self) -> &'static str {
        match self.lang {
            Language::English => "Description",
            Language::Vietnamese => "Mô tả",
        }
    }

    pub fn item_price(&self) -> &'static str {
        match self.lang {
            Language::English => "Price (won)",
            Language::Vietnamese => "Giá (won)",
        }
    }

    pub fn item_category(&self) -> &'static str {
        match self.lang {
            Language::English => "Category",
            Language::Vietnamese => "Danh mục",
        }
    }

    pub fn delete(&self) -> &'static str {
        match self.lang {
            Language::English => "Delete",
            Language::Vietnamese => "Xóa",
        }
    }

    pub fn edit(&self) -> &'static str {
        match self.lang {
            Language::English => "Edit",
            Language::Vietnamese => "Chỉnh sửa",
        }
    }

    pub fn clear_cart(&self) -> &'static str {
        match self.lang {
            Language::English => "Clear Cart",
            Language::Vietnamese => "Xóa giỏ hàng",
        }
    }

    pub fn generate_list(&self) -> &'static str {
        match self.lang {
            Language::English => "Generate List",
            Language::Vietnamese => "Tạo danh sách",
        }
    }

    pub fn copied_notification(&self) -> &'static str {
        match self.lang {
            Language::English => "Copied to clipboard!",
            Language::Vietnamese => "Đã sao chép vào bộ nhớ đệm!",
        }
    }

    pub fn settings(&self) -> &'static str {
        match self.lang {
            Language::English => "Settings",
            Language::Vietnamese => "Cài đặt",
        }
    }
}
