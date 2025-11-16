# Market Master - Quick Start Guide

## Building & Running

### Build Release Version
```bash
cargo build --release
```

### Run Application
```bash
cargo run --release
```

The app will start in fullscreen maximized mode.

---

## User Interface Overview

### Layout
The application uses a three-panel layout:
- **Left Panel**: Category filter buttons
- **Center Panel**: Item cards (searchable, filterable)
- **Right Panel**: Shopping cart with totals

### Header Controls
- **🔍 Search Bar**: Find items by name or code
- **EN / VI**: Switch language (English/Vietnamese)
- **⚙ Settings**: Open menu with:
  - 📋 Item Manager (manage inventory)
  - 📝 Edit Raw List (edit raw-list.txt directly)
  - 🎨 Theme selector (Light, Dark, Warm)

---

## Building an Order

### Step 1: Select Category or Search
- Click category buttons on left (🐖 Pork, 🐓 Chicken, etc.)
- Or type in search bar to find specific items
- Click "All Items" to see everything

### Step 2: Add Items
- Click **"➕ Add"** to add item with quantity 1
- Or type quantity directly in the **[ input ]** box
- Use **[ - ]** and **[ + ]** to adjust

### Step 3: Review Cart
- Right panel shows all selected items
- Displays quantity, price, and subtotal
- Shows total cost at bottom

### Step 4: Export Order
1. Click **"Generate List"** to format the order
2. Click **"Copy to Clipboard"** to copy to clipboard
3. Paste into order system, email, etc.

### Step 5: Clear When Done
- Click **"Clear Cart"** to empty selections
- Ready for next order

---

## Switching Themes

The app comes with three color themes:
- **Light** (default): Clean, bright, professional
- **Dark**: Easy on eyes, great for low-light environments
- **Warm**: Warm tones, easier on the eyes than bright white

### Change Theme
1. Click **⚙ Settings** (top right)
2. Select **🎨 Theme:**
3. Choose: Light, Dark, or Warm
4. Theme applies immediately

---

## Managing Inventory

### Access Item Manager
1. Click **⚙ Settings** button (top right)
2. Select **📋 Item Manager**
3. Item Manager modal opens

### Add New Item
1. Click **"Add Item"** button
2. Fill in form fields:
   - **Item Code**: Unique identifier (e.g., "CK01")
   - **Description**: Item name in Vietnamese/English
   - **Price**: Price in won (numeric only)
   - **Category**: Select from dropdown
3. Changes save automatically

### Edit Existing Item
1. Find item in the table
2. Click **✏ Edit** button
3. Modify fields
4. Changes save automatically

### Delete Item
1. Find item in the table
2. Click **🗑 Delete** button
3. Item removed immediately

---

## Tips & Tricks

### Fast Quantity Entry
- Click the quantity number to directly type amount
- Press Enter to confirm
- Set to 0 to remove item

### Search Tips
- Type item code (e.g., "CK01") to find instantly
- Type part of description (e.g., "gà" for chicken)
- Works with Vietnamese characters

### Category Filtering
- Click category once to filter
- Click again to maintain filter + search
- "All Items" button resets to full list

### Keyboard Shortcuts
- Tab through search bar → category buttons → item cards
- Enter to confirm quantity input
- Alt+Tab to switch windows

---

## Data Files

### items.csv
- Main inventory file (139 items)
- Columns: code, description, price, category
- Auto-updated when using Item Manager

### raw-list.txt
- Alternative format with emoji headers
- Can be edited manually if needed
- Re-parsed on save (for bulk updates)

---

## Troubleshooting

### Items Not Showing
- Check if category filter is restricting view
- Try clearing search box
- Click "All Items" button

### Quantity Won't Update
- Click the number field to activate input
- Type number and press Enter
- Use arrow buttons if text input fails

### Can't Save Item
- Ensure code is unique
- Price must be numeric (no spaces)
- Category must be selected

### Vietnamese Text Not Showing
- Font loading attempted at startup
- Check console output for font status
- System fonts used as fallback

---

## Keyboard Navigation

| Key | Action |
|-----|--------|
| Tab | Navigate between fields |
| Enter | Confirm input/button |
| Esc | Close modal/dialog |
| Ctrl+C | Copy to clipboard (if text selected) |

---

## Contact & Support

For issues or feature requests:
1. Check IMPLEMENTATION_SUMMARY.md for technical details
2. Review code comments in src/ui.rs
3. Examine data in items.csv for format reference

---

## Features Checklist

✅ Three-panel responsive layout
✅ Category filtering (8 categories)
✅ Real-time search across all items
✅ Direct quantity input via text field
✅ Shopping cart with totals
✅ Order export (Generate List)
✅ Clipboard integration
✅ Item management (CRUD)
✅ Vietnamese language support
✅ Professional color scheme
✅ Emoji icons throughout
✅ Notification toasts
✅ CSV data persistence
✅ Raw list format support

---

## Keyboard Input Examples

**Search for items:**
```
🔍 Search: "nạc"  → Shows all pork cuts with "nạc"
🔍 Search: "CK"   → Shows all chicken items starting with CK
```

**Add to cart:**
```
Click "Add" → Quantity becomes 1
Click in qty field → Type "5" → Enter → Quantity now 5
Click [ - ] → Quantity becomes 4
```

**Manage inventory:**
```
⚙ Settings → Item Manager
[Add Item] → Fill form → Auto-saves
[✏] Edit existing → Modify → Auto-saves
[🗑] Delete → Removes instantly
```

---

*Version: 1.0 (Complete UI/UX Revamp)*
*Last Updated: November 2025*
