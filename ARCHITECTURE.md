# Market Master - Architecture & Technical Design

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      eframe (Desktop Window)                 │
├─────────────────────────────────────────────────────────────┤
│                  egui (Immediate Mode Renderer)              │
├─────────────────────────────────────────────────────────────┤
│                     MyApp (State Manager)                    │
├───────────────┬────────────────────────┬────────────────────┤
│ Categories    │   Item Browser         │  Shopping Cart     │
│ (Filtering)   │   (Search & Display)   │  (Selection View)  │
├───────────────┴────────────────────────┴────────────────────┤
│              Data Layer (CSV + Memory)                      │
│  items.csv ←→ MyApp.items ←→ Item Manager Modal            │
└─────────────────────────────────────────────────────────────┘
```

---

## Component Structure

### Core Modules

#### `main.rs`
- **Purpose**: Application entry point and window setup
- **Responsibilities**:
  - Initialize eframe with native options
  - Load Vietnamese/emoji fonts
  - Create MyApp instance
  - Handle window maximization

#### `app.rs`
- **Purpose**: Application state and business logic
- **Key Structures**:
  ```rust
  pub struct MyApp {
      pub items: Vec<Item>,           // Inventory
      pub search: String,              // Search query
      pub selected: HashMap<String, i32>, // Cart (code → qty)
      pub selected_category: ItemCategory, // Filter
      pub show_item_manager: bool,    // Modal toggle
      pub notification: Option<(String, f32)>, // Toast
      // ... other fields
  }
  ```
- **Key Methods**:
  - `new()`: Initialize from CSV files
  - `parse_items()`: CSV to Item struct
  - `parse_raw_list()`: Raw text to Items with auto-categorization
  - `save_items_to_csv()`: Persist inventory changes

#### `models.rs`
- **Purpose**: Data structures and types
- **Key Types**:
  ```rust
  pub struct Item {
      pub code: String,
      pub description: String,
      pub price: u32,
      pub category: String,
  }
  
  pub enum ItemCategory {
      Pork, Chicken, Duck, Beef,
      Seafood, Sauces, Vegetables, Other
  }
  
  pub enum Language {
      English, Vietnamese
  }
  ```
- **Serialization**: All structs use `serde` for CSV compatibility

#### `ui.rs`
- **Purpose**: Complete UI implementation
- **Architecture**:
  ```
  Main Update Loop
  ├─ Notification system (timer)
  ├─ TopBottomPanel (Header)
  │  ├─ Search bar
  │  ├─ Language toggle
  │  └─ Settings button
  ├─ SidePanel Left (Categories)
  │  └─ Category buttons
  ├─ SidePanel Right (Cart)
  │  ├─ Item list
  │  ├─ Totals
  │  └─ Actions
  ├─ CentralPanel (Item Browser)
  │  └─ Item cards grid
  └─ Modal Windows
     ├─ Item Manager
     └─ Raw Editor (legacy)
  ```

#### `i18n.rs`
- **Purpose**: Internationalization strings
- **Structure**:
  ```rust
  pub struct Translations {
      pub lang: Language,
  }
  ```
- **Supported Languages**: English, Vietnamese
- **Method Pattern**: `trans.string_key()` returns &'static str

#### `fonts.rs`
- **Purpose**: Font loading for Vietnamese/emoji support
- **Features**:
  - Tries system fonts first (Arial, Segoe UI, DejaVuSans)
  - Falls back to bundled NotoColorEmoji font
  - Cross-platform support (Windows, macOS, Linux)

---

## Data Flow

### User Adds Item to Cart

```
User clicks "Add" button
    ↓
item_card() function handles click
    ↓
Updates MyApp.selected HashMap
    key: item.code
    value: quantity (starts at 1)
    ↓
Shopping Cart displays updated total
    ↓
UI re-renders (egui automatic)
```

### User Searches for Item

```
User types in search bar
    ↓
MyApp.search field updates
    ↓
item_browser() reads search query
    ↓
Filters items by:
  - Code matches (case-insensitive)
  - Description matches (Vietnamese + English)
  - Category matches (if filtered)
    ↓
Filtered results displayed in center panel
```

### User Generates Order

```
User clicks "Generate List"
    ↓
Loop through MyApp.selected HashMap
    ↓
For each (code, quantity) pair:
  - Find item in MyApp.items
  - Format: "CK01 đùi gà (1 phần)"
    ↓
Join lines with newlines
    ↓
Store in MyApp.print_output
    ↓
Display in right panel text area
```

### User Saves Item Changes

```
Item Manager modal open
    ↓
User clicks Edit/Delete/Add
    ↓
Changes made to MyApp.items
    ↓
MyApp.save_items_to_csv() called
    ↓
CSV Writer serializes all items
    ↓
File written to items.csv
    ↓
On next session: parse_items() reads updated CSV
```

---

## State Management

### MyApp Fields and Purposes

| Field | Type | Purpose |
|-------|------|---------|
| `items` | `Vec<Item>` | Master inventory list |
| `search` | `String` | Current search query |
| `selected` | `HashMap<String, i32>` | Cart contents (code→qty) |
| `selected_category` | `ItemCategory` | Active category filter |
| `language` | `Language` | UI language setting |
| `show_raw_editor` | `bool` | Legacy raw editor visibility |
| `raw_content` | `String` | Raw list file content |
| `show_item_manager` | `bool` | Item Manager modal visibility |
| `edit_item_*` | `String/ItemCategory` | Form fields for editing |
| `notification` | `Option<(String, f32)>` | Toast message + time |
| `print_output` | `String` | Formatted order output |

### State Mutations

```
User Input → Field Update → Re-render
    ↓
Quantity stepper
    ↓
MyApp.selected.insert(code, qty)
    ↓
Cart total recalculated
    ↓
Right panel re-renders

Search box
    ↓
MyApp.search = new_value
    ↓
item_browser() filters with new query
    ↓
Center panel re-renders

Category button
    ↓
MyApp.selected_category = new_cat
    ↓
item_browser() filters by category
    ↓
Center panel re-renders
```

---

## UI Rendering Flow

### Three-Panel Layout Hierarchy

```
egui::TopBottomPanel::top("header")
  └─ Header controls (search, language, settings)
     
egui::SidePanel::left("category_panel")
  └─ Category filter buttons
     
egui::SidePanel::right("cart_panel")
  └─ shopping_cart_panel()
     ├─ Cart items
     ├─ Totals
     ├─ Generate/Clear buttons
     └─ Print output
     
egui::CentralPanel::default()
  └─ item_browser()
     └─ ScrollArea with ItemCard grid
        └─ item_card() per item
           ├─ Code + Description
           ├─ Price
           └─ Quantity stepper
```

### Responsive Behavior

- **Panel Widths**:
  - Left: 150px (resizable)
  - Right: 300px (resizable)
  - Center: Remaining space
  
- **Scroll Areas**: Auto-shrink [false, false] for independent scrolling

- **Height Constraints**:
  - Item cards: min_height 100px
  - Cart items: max_height 400px
  - Print area: desired_rows 5

---

## Color System Implementation

```rust
const COLOR_BACKGROUND: Color32 = Color32::from_rgb(248, 249, 250);
const COLOR_CARD: Color32 = Color32::from_rgb(255, 255, 255);
const COLOR_PRIMARY: Color32 = Color32::from_rgb(74, 144, 226);
const COLOR_SUCCESS: Color32 = Color32::from_rgb(80, 227, 194);
const COLOR_DANGER: Color32 = Color32::from_rgb(233, 78, 119);
const COLOR_TEXT_SECONDARY: Color32 = Color32::from_rgb(119, 119, 119);
```

### Color Usage Mapping

| Element | Color | Reason |
|---------|-------|--------|
| Prices | PRIMARY | Emphasis, monetary value |
| Add buttons | SUCCESS | Positive action |
| Delete buttons | DANGER | Destructive action |
| Notifications | SUCCESS | Confirmation feedback |
| Item codes | SECONDARY | Secondary information |
| Backgrounds | BACKGROUND | Low contrast, restful |

---

## CSV Data Format

### items.csv Structure
```csv
code,description,price,category
CK01,đùi gà 1kg,5900,Chicken
PG01,thịt mông 1kg,6500,Pork
SE01,Tôm 1kg,14000,Seafood
SC01,MẮM NÊM THÁI LAN 350ml,3100,Sauces
```

### Parsing Logic
1. Read CSV file with csv crate
2. Deserialize each row into Item struct
3. Category field populated automatically
4. Fallback to empty string if missing

### Writing Logic
1. Create CSV writer
2. Serialize each Item struct
3. Convert to String
4. Write to file atomically

---

## Filtering Algorithm

### Search Filter
```rust
let matches_search = query.is_empty()
    || item.code.to_lowercase().contains(&query)
    || item.description.to_lowercase().contains(&query);
```
- O(n·m) complexity where n=items, m=query length
- Case-insensitive matching
- Supports Vietnamese diacritics

### Category Filter
```rust
let matches_category = selected_category == ItemCategory::Other
    || ItemCategory::from_str(&item.category) == selected_category;
```
- O(n) iteration over items
- String to enum comparison
- Default "Other" shows all items

### Combined Filter
```rust
items.iter()
    .filter(|item| matches_search && matches_category)
    .collect()
```
- Both conditions must be true
- Applied in sequence

---

## Notification System

### Implementation
```rust
pub notification: Option<(String, f32)>

// In update():
if let Some((_, time_left)) = &mut self.notification {
    *time_left -= ctx.input(|i| i.unstable_dt);
    if *time_left <= 0.0 {
        self.notification = None;
    }
}
```

### Usage
```rust
app.notification = Some((trans.copied_notification().to_string(), 2.0));
// Displays for 2 seconds then disappears
```

### Rendering
- Positioned at CENTER_BOTTOM
- Area widget with custom anchor
- Green success color

---

## Error Handling

### File I/O
```rust
// CSV parsing
let items = Self::parse_items(&csv_content);
// Panics on parse error (for now)

// Saving
match std::fs::write("items.csv", data) {
    Ok(_) => eprintln!("Saved"),
    Err(e) => eprintln!("Error: {}", e)
}
```

### Input Validation
```rust
// Quantity input
if let Ok(new_amt) = qty_str.parse::<i32>() {
    // Valid integer
} else {
    // Silent fail - keeps existing value
}
```

---

## Performance Considerations

### Memory Usage
- Items vector: ~140 items × ~150 bytes = ~21KB
- Selected HashMap: ~20 items max × ~20 bytes = ~400B
- Notification option: ~64 bytes
- String buffers: ~1KB

### CPU Usage
- Filter operation: Called on every frame
- Search: Linear scan through items
- Rendering: egui handles efficiently
- Frame rate: 60 FPS target

### Optimization Techniques
1. Cloned items only when necessary (filter operation)
2. HashMap for O(1) cart lookups
3. Scroll areas for large lists
4. Early exits in filters

---

## Extension Points

### Adding New Categories
1. Add variant to `ItemCategory` enum
2. Update `as_str()` method
3. Update `from_str()` parsing
4. Update `all()` array
5. Update raw parser logic

### Adding New Languages
1. Create new `Language` enum variant
2. Add translation methods in `Translations`
3. Add language toggle in header
4. Update i18n.rs with strings

### Adding New Features
1. Extend `MyApp` struct with new field
2. Add UI component function (e.g., `fn my_feature_panel()`)
3. Call from appropriate panel in `update()`
4. Update translations if user-facing

---

## Testing Strategy

### Unit Testing
- CSV parsing: Verify items loaded correctly
- Filtering: Check search/category logic
- Calculation: Verify totals are accurate

### Integration Testing
- Full workflow: Add → Generate → Copy
- Data persistence: Save and reload
- Language switching: All strings load
- Filtering: Search + category combinations

### Manual Testing
- Visual layout: Responsive, colors correct
- Input handling: All fields work
- Edge cases: Empty cart, no results, etc.
- Performance: Smooth scrolling, no lag

---

## Deployment

### Build
```bash
cargo build --release
```
Produces single executable in `target/release/`

### Distribution
- Windows: `.exe` file
- Linux: Binary executable
- macOS: App bundle

### System Requirements
- Windows 10+ / Linux / macOS 10.12+
- 50MB disk space
- ~100MB RAM while running

---

## Future Refactoring Opportunities

1. **Database Migration**: Move from CSV to SQLite
2. **Component Library**: Extract UI components to separate module
3. **Service Layer**: Separate business logic from state
4. **Config File**: Load settings from config instead of hardcoded
5. **Async Loading**: Load items in background thread
6. **Caching**: Cache filtered results

---

*Architecture Version: 1.0*
*Updated: November 2025*
