# The Market Siêu thị việt hàn

A modern Rust desktop application for managing grocery shopping lists at Vietnamese-Korean supermarkets. Built with egui for a clean, intuitive user interface with full theme and language support.

![Application Screenshot](https://img.shields.io/badge/Status-Active-brightgreen) ![Rust](https://img.shields.io/badge/Rust-1.70+-orange) ![egui](https://img.shields.io/badge/egui-0.27-blue)

## 🌟 Features

### Core Shopping Features
- **📦 Item Catalog**: Browse 130+ grocery items across 8 categories
- **🛒 Shopping Cart**: Add items with quantity controls and real-time totals
- **🔍 Smart Search**: Global search across item codes and descriptions
- **📋 List Generation**: Create formatted shopping lists for printing or copying
- **💾 Data Persistence**: Automatic saving and loading of cart state

### Category Organization
- **🐖 Pork**: All pork products and cuts
- **🐓 Chicken**: Chicken pieces, organs, and processed products
- **🦆 Duck**: Whole ducks and duck parts
- **🐂 Beef**: Beef cuts, organs, and ground meat
- **🐟 Seafood**: Fish, shrimp, squid, and seafood specialties
- **🥫 Sauces**: Vietnamese and Korean sauces, seasonings, and condiments
- **🍐 Vegetables**: Fresh produce and vegetables
- **Other**: Miscellaneous items that don't fit other categories

### User Interface
- **🎨 Three Themes**: Light, Dark, and Warm color schemes
- **🌍 Bilingual**: Full Vietnamese and English support
- **📱 Responsive Layout**: Three-panel design (categories, items, cart)
- **⚙️ Settings Menu**: Access to item manager and raw list editor
- **🔔 Notifications**: Toast messages for user feedback

### Advanced Features
- **📊 Item Manager**: Add, edit, and delete items from the catalog
- **📝 Raw List Editor**: Import/export items from text files
- **🎯 Theme-aware Styling**: Intelligent contrast adjustment across themes
- **🔤 Font Support**: Built-in Vietnamese and emoji font support
- **💡 Smart Categorization**: Automatic item categorization from text imports

## 🚀 Getting Started

### Prerequisites
- Rust 1.70 or later
- Cargo package manager

### Installation & Running

1. **Clone or extract the project**
   ```bash
   cd market-shopping-app
   ```

2. **Run the application**
   ```bash
   cargo run
   ```

The application will start maximized and automatically load:
- Default theme: Light mode
- Default language: Vietnamese
- Items database: `items.csv`

### Building for Distribution
```bash
cargo build --release
```

## 📁 Project Structure

```
src/
├── app.rs          # Main application logic and state management
├── ui.rs           # User interface rendering and layout
├── models.rs       # Data models, enums, and theme definitions
├── i18n.rs         # Vietnamese/English translations
├── fonts.rs        # Font loading and Vietnamese text support
└── main.rs         # Application entry point

data/
├── items.csv       # Main product database (130+ items)
├── raw-list.txt    # Alternative text-based import format
└── fonts/
    └── NotoColorEmoji.ttf  # Bundled emoji font support
```

## 📊 Data Format

### CSV Format (`items.csv`)
```csv
code,description,price,category
CK01,đùi gà 1kg,5900,Chicken
PG01,thịt mông 1kg,6500,Pork
SE01,Tôm 1kg,14000,Seafood
```

### Raw Text Format (`raw-list.txt`)
The raw list parser handles flexible formats:
```
🐖 Pork
PG01 thịt mông 1kg / 6,500
PG02 thịt nạc lộn mỡ 1kg / 5,700

🐓 Chicken
CK01 đùi gà 1kg / 5,900 → 5,900
CK02 cánh gà 1kg / 8,000
```

## 🎨 Customization

### Adding New Items
1. Use the built-in **Item Manager** (⚙️ Settings → 📋 Item Manager)
2. Or edit `items.csv` directly
3. Or use the **Raw List Editor** for bulk imports

### Creating Custom Themes
Themes are defined in `src/models.rs`:
```rust
Theme::Light => ThemeColors {
    background: egui::Color32::from_rgb(248, 249, 250),
    card: egui::Color32::from_rgb(255, 255, 255),
    primary: egui::Color32::from_rgb(74, 144, 226),
    // ... other colors
}
```

### Adding New Languages
Language support is in `src/i18n.rs`. Add new translation methods:
```rust
pub fn new_feature(&self) -> &'static str {
    match self.lang {
        Language::English => "New Feature",
        Language::Vietnamese => "Tính năng mới",
        // Add new language here
    }
}
```

## 🔧 Technical Details

### Dependencies
- **efui 0.27**: Cross-platform GUI framework
- **egui 0.27**: Immediate mode GUI library
- **csv 1.1**: CSV parsing and generation
- **serde 1.0**: Data serialization/deserialization

### Architecture Highlights
- **Immediate Mode GUI**: No retained state, efficient rendering
- **Theme-aware Components**: Smart contrast calculation for readability
- **Bilingual Architecture**: Complete Vietnamese/English support
- **Multi-format Import**: Flexible text parsing for various data sources
- **Responsive Design**: Adaptive layout for different screen sizes

### Performance Features
- **Optimized Rendering**: Efficient UI updates with minimal redraws
- **Smart Search**: Real-time filtering without database queries
- **Memory Efficient**: Compact data structures and lazy loading
- **Cross-platform**: Native performance on Windows, macOS, and Linux

## 🌐 Language Support

### Vietnamese (Tiếng Việt)
- **Default language** with full localization
- **Vietnamese font support** with automatic fallback
- **Cultural localization** with appropriate terminology

### English
- **Complete translation** of all interface elements
- **Korean supermarket context** maintained in both languages
- **User-friendly terminology** for international users

## 📝 Usage Guide

### Basic Shopping Workflow
1. **Browse Categories**: Click category buttons on the left panel
2. **Search Items**: Use the search field in the header
3. **Add to Cart**: Click "➕ Add" or set quantities with +/- buttons
4. **View Cart**: Check your selections in the right panel
5. **Generate List**: Click "Generate List" to create formatted output
6. **Copy/Print**: Copy to clipboard or print the shopping list

### Advanced Features
- **Multi-quantity Selection**: Click quantity fields to edit amounts directly
- **Category Filtering**: Combine search with category selection
- **Bulk Operations**: Use Raw List Editor for large inventory updates
- **Theme Switching**: Change appearance via ⚙️ Settings → 🎨 Theme

## 🛠️ Development

### Code Organization
- **Modular Design**: Separation of concerns between UI, logic, and data
- **Type Safety**: Strong typing throughout with Rust's safety guarantees
- **Error Handling**: Graceful handling of file I/O and parsing errors
- **Scalable Architecture**: Easy to add new features and languages

### Contributing
1. Fork the repository
2. Create a feature branch
3. Make your changes with proper testing
4. Submit a pull request

## 📄 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

- **egui Team**: For the excellent immediate mode GUI framework
- **Rust Community**: For the robust and safe programming language
- **Vietnamese-Korean Supermarkets**: For the inspiration and real-world use case
- **Unicode Consortium**: For emoji and international text support

---

**Built with ❤️ for the Vietnamese-Korean community**

For support or questions, please refer to the source code or create an issue in the project repository.