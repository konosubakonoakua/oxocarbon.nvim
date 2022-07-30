/*
                             O X O C A R B O N
       _..._         _..._         _..._         _..._         _..._
     .:::::::.     .::::. `.     .::::  `.     .::'   `.     .'     `.
    :::::::::::   :::::::.  :   ::::::    :   :::       :   :         :
    :::::::::::   ::::::::  :   ::::::    :   :::       :   :         :
    `:::::::::'   `::::::' .'   `:::::   .'   `::.     .'   `.       .'
      `':::''       `'::'-'       `'::.-'       `':..-'       `-...-'

  Colorscheme name:    oxocarbon: oxidized carbon
  Description:         Neovim Colorscheme inspired from the IBM Carbon Palette in rust with nvim-oxi
  Author:              https://github.com/shaunsingh

  Losely based off of IBM Carbon Palette
  https://www.ibm.com/brand/experience-guides/developer/brand/color/)

*/

use nvim_oxi::{self as oxi, api, opts::*};

#[oxi::module]
fn oxocarbon() -> oxi::Result<String> {
    // set termguicolors
    api::set_option("termguicolors", true)?;

    // decide pallete based on value of vim background.
    // Each palette consists of 16 base colors, 1 blend color (used for contrast for floating menus and such), and 1 transparent color.
    let oxocarbon: [&str; 18] = match api::get_option::<String>("background").unwrap().as_str() {
        "dark" => ["#161616", "#262626", "#393939", "#525252", 
                   "#dde1e6", "#f2f4f8", "#ffffff", 
                   "#08bdba", "#3ddbd9", "#78a9ff", 
                   "#ee5396", "#33b1ff", "#ff7eb6", "#42be65", "#be95ff", "#82cfff", 
                   "#131313", ""],
        "light" => ["#FFFFFF", "#FAFAFA", "#ECEFF1", 
                    "#161616", "#37474F", "#90A4AE", "#525252", 
                    "#08bdba", "#ff7eb6", "#ee5396", 
                    "#FF6F00", "#0f62fe", "#673AB7", "#42be65", "#be95ff", "#FFAB91", 
                    "#FAFAFA", ""],
        _ => panic!("Error: Background not set"),
    };

    // terminal
    api::set_var("terminal_color_0", oxocarbon[0].to_string())?;
    api::set_var("terminal_color_1", oxocarbon[1].to_string())?;
    api::set_var("terminal_color_2", oxocarbon[2].to_string())?;
    api::set_var("terminal_color_3", oxocarbon[3].to_string())?;
    api::set_var("terminal_color_4", oxocarbon[4].to_string())?;
    api::set_var("terminal_color_5", oxocarbon[5].to_string())?;
    api::set_var("terminal_color_6", oxocarbon[6].to_string())?;
    api::set_var("terminal_color_7", oxocarbon[7].to_string())?;
    api::set_var("terminal_color_8", oxocarbon[8].to_string())?;
    api::set_var("terminal_color_9", oxocarbon[9].to_string())?;
    api::set_var("terminal_color_10", oxocarbon[10].to_string())?;
    api::set_var("terminal_color_11", oxocarbon[11].to_string())?;
    api::set_var("terminal_color_12", oxocarbon[12].to_string())?;
    api::set_var("terminal_color_13", oxocarbon[13].to_string())?;
    api::set_var("terminal_color_14", oxocarbon[14].to_string())?;
    api::set_var("terminal_color_15", oxocarbon[15].to_string())?;

    // editor
    api::set_hl(0, "ColorColumn", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "Cursor", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[4]).build()))?;
    api::set_hl(0, "CursorLine", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "CursorColumn", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "CursorLineNr", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Error", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[11]).build()))?;
    api::set_hl(0, "LineNr", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "MatchParen", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[2]).underline(true).build()))?;
    api::set_hl(0, "NonText", Some(&SetHighlightOpts::builder().fg(oxocarbon[2]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Normal", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "Pmenu", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "PmenuSbar", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "PmenuSel", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "PmenuThumb", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "SpecialKey", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Visual", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "VisualNOS", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[2]).build()))?;

    // diagnostics
    api::set_hl(0, "DiagnosticWarn", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "DiagnosticError", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "DiagnosticInfo", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "DiagnosticHint", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "DiagnosticUnderlineWarn", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).undercurl(true).build()))?;
    api::set_hl(0, "DiagnosticUnderlineError", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).undercurl(true).build()))?;
    api::set_hl(0, "DiagnosticUnderlineInfo", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).undercurl(true).build()))?;
    api::set_hl(0, "DiagnosticUnderlineHint", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).undercurl(true).build()))?;

    // lsp
    api::set_hl(0, "LspReferenceText", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[3]).build()))?;
    api::set_hl(0, "LspReferenceread", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[3]).build()))?;
    api::set_hl(0, "LspReferenceWrite", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[3]).build()))?;
    api::set_hl(0, "LspSignatureActiveParameter", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;

    // gutter
    api::set_hl(0, "Folded", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "FoldColumn", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "SignColumn", Some(&SetHighlightOpts::builder().fg(oxocarbon[1]).bg(oxocarbon[0]).build()))?;

    // navigation
    api::set_hl(0, "Directory", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;

    // prompts
    api::set_hl(0, "EndOfBuffer", Some(&SetHighlightOpts::builder().fg(oxocarbon[1]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "ErrorMsg", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[11]).build()))?;
    api::set_hl(0, "ModeMsg", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "MoreMsg", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Question", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "WarningMsg", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[13]).build()))?;
    api::set_hl(0, "WildMenu", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[1]).build()))?;

    // search
    api::set_hl(0, "IncSearch", Some(&SetHighlightOpts::builder().fg(oxocarbon[6]).bg(oxocarbon[10]).build()))?;
    api::set_hl(0, "Search", Some(&SetHighlightOpts::builder().fg(oxocarbon[1]).bg(oxocarbon[8]).build()))?;

    // tabs
    api::set_hl(0, "TabLine", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "TabLineFill", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "TabLineSel", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[3]).build()))?;

    // window
    api::set_hl(0, "Title", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "VertSplit", Some(&SetHighlightOpts::builder().fg(oxocarbon[1]).bg(oxocarbon[0]).build()))?;

    // regular syntax
    api::set_hl(0, "Boolean", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Character", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Comment", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Conceal", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Conditional", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Constant", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Decorator", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Define", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Delimeter", Some(&SetHighlightOpts::builder().fg(oxocarbon[6]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Exception", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Float", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Function", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Identifier", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Include", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Keyword", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Label", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Number", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Operator", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "PreProc", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Repeat", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Special", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "SpecialChar", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "SpecialComment", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Statement", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "StorageClass", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "String", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Structure", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Tag", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Todo", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Type", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "Typedef", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;

    // treesitter
    api::set_hl(0, "TSAnnotation", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSAttribute", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSBoolean", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSCharacter", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSComment", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[17]).italic(true).build()))?;
    api::set_hl(0, "TSConstructor", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSConditional", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSConstant", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSConstBuiltin", Some(&SetHighlightOpts::builder().fg(oxocarbon[7]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSConstMacro", Some(&SetHighlightOpts::builder().fg(oxocarbon[7]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSError", Some(&SetHighlightOpts::builder().fg(oxocarbon[11]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSException", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSField", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSFloat", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSFunction", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).bold(true).build()))?;
    api::set_hl(0, "TSFuncBuiltin", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSFuncMacro", Some(&SetHighlightOpts::builder().fg(oxocarbon[7]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSInclude", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSKeyword", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSKeywordFunction", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSKeywordOperator", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSLabel", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSMethod", Some(&SetHighlightOpts::builder().fg(oxocarbon[7]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSNamespace", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSNumber", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSOperator", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSParameter", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSParameterReference", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSProperty", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSPunctDelimiter", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSPunctBracket", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSPunctSpecial", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSRepeat", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSString", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSStringRegex", Some(&SetHighlightOpts::builder().fg(oxocarbon[7]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSStringEscape", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSSymbol", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).bold(true).build()))?;
    api::set_hl(0, "TSTag", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSTagDelimiter", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSText", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSStrong", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[17]).bold(true).build()))?;
    api::set_hl(0, "TSEmphasis", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).bold(true).build()))?;
    api::set_hl(0, "TSUnderline", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).underline(true).build()))?;
    api::set_hl(0, "TSStrike", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).strikethrough(true).build()))?;
    api::set_hl(0, "TSTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSLiteral", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSURI", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).underline(true).build()))?;
    api::set_hl(0, "TSType", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSTypeBuiltin", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSVariable", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSVariableBuiltin", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "TSCurrentScope", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[17]).bold(true).build()))?;
    api::set_hl(0, "TreesitterContext", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[1]).build()))?;

    // neovim
    api::set_hl(0, "NvimInternalError", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[8]).build()))?;
    api::set_hl(0, "NormalFloat", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[16]).build()))?;
    api::set_hl(0, "FloatBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[16]).bg(oxocarbon[16]).build()))?;
    api::set_hl(0, "NormalNC", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "TermCursor", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[4]).build()))?;
    api::set_hl(0, "TermCursorNC", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[4]).build()))?;

    // statusline/winbar
    api::set_hl(0, "StatusLine", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "WinBar", Some(&SetHighlightOpts::builder().fg("#a2a9b0").bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "StatusNormal", Some(&SetHighlightOpts::builder().fg("#a2a9b0").bg(oxocarbon[0]).underline(true).build()))?;
    api::set_hl(0, "StatusCommand", Some(&SetHighlightOpts::builder().fg("#a2a9b0").bg(oxocarbon[0]).underline(true).build()))?;
    api::set_hl(0, "StatusPosition", Some(&SetHighlightOpts::builder().fg("#a2a9b0").bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "StatusReplace", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[8]).build()))?;
    api::set_hl(0, "StatusInsert", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[12]).build()))?;
    api::set_hl(0, "StatusVisual", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[14]).build()))?;
    api::set_hl(0, "StatusTerminal", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[11]).build()))?;
    api::set_hl(0, "StatusLineDiagnosticWarn", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[0]).bold(true).build()))?;
    api::set_hl(0, "StatusLineDiagnosticError", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[0]).bold(true).build()))?;

    // telescope
    api::set_hl(0, "TelescopeBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[16]).bg(oxocarbon[16]).build()))?;
    api::set_hl(0, "TelescopePromptBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[2]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "TelescopePromptNormal", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "TelescopePromptPrefix", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "TelescopeNormal", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[16]).build()))?;
    api::set_hl(0, "TelescopePreviewTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[2]).bg(oxocarbon[11]).build()))?;
    api::set_hl(0, "TelescopePromptTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[2]).bg(oxocarbon[8]).build()))?;
    api::set_hl(0, "TelescopeResultsTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[16]).bg(oxocarbon[16]).build()))?;
    api::set_hl(0, "TelescopeSelection", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "TelescopePreviewLine", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[1]).build()))?;

    // notify
    api::set_hl(0, "NotifyERRORBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyWARNBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyINFOBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyDEBUGBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyTRACEBorder", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyERRORIcon", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyWARNIcon", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyINFOIcon", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyDEBUGIcon", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyTRACEIcon", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyERRORTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyWARNTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyINFOTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyDEBUGTitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NotifyTRACETitle", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;

    // cmp
    api::set_hl(0, "CmpItemAbbr", Some(&SetHighlightOpts::builder().fg("#adadad").bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemAbbrMatch", Some(&SetHighlightOpts::builder().fg(oxocarbon[5]).bg(oxocarbon[17]).bold(true).build()))?;
    api::set_hl(0, "CmpItemAbbrMatchFuzzy", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindInterface", Some(&SetHighlightOpts::builder().fg(oxocarbon[11]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindText", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindVariable", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindProperty", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindKeyword", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindUnit", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindFunction", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "CmpItemKindMethod", Some(&SetHighlightOpts::builder().fg(oxocarbon[7]).bg(oxocarbon[17]).build()))?;

    // nvimtree
    api::set_hl(0, "NvimTreeImageFile", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NvimTreeFolderIcon", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NvimTreeWinSeperator", Some(&SetHighlightOpts::builder().fg(oxocarbon[0]).bg(oxocarbon[0]).build()))?;
    api::set_hl(0, "NvimTreeFolderName", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NvimTreeIndentMarker", Some(&SetHighlightOpts::builder().fg(oxocarbon[2]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NvimTreeEmptyFolderName", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NvimTreeOpenedFolderName", Some(&SetHighlightOpts::builder().fg(oxocarbon[15]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NvimTreeNormal", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[16]).build()))?;

    // neogit
    api::set_hl(0, "NeogitBranch", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NeogitRemote", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "NeogitDiffAddHighlight", Some(&SetHighlightOpts::builder().fg(oxocarbon[13]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "NeogitDiffDeleteHighlight", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "NeogitDiffContextHighlight", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[1]).build()))?;
    api::set_hl(0, "NeogitHunkHeader", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[2]).build()))?;
    api::set_hl(0, "NeogitHunkHeaderHighlight", Some(&SetHighlightOpts::builder().fg(oxocarbon[4]).bg(oxocarbon[3]).build()))?;

    // gitsigns
    api::set_hl(0, "GitSignsAdd", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "GitSignsChange", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "GitSignsDelete", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;

    // parinfer
    api::set_hl(0, "Trailhighlight", Some(&SetHighlightOpts::builder().fg(oxocarbon[3]).bg(oxocarbon[17]).build()))?;

    // hydra
    api::set_hl(0, "HydraRed", Some(&SetHighlightOpts::builder().fg(oxocarbon[12]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "HydraBlue", Some(&SetHighlightOpts::builder().fg(oxocarbon[9]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "HydraAmaranth", Some(&SetHighlightOpts::builder().fg(oxocarbon[10]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "HydraTeal", Some(&SetHighlightOpts::builder().fg(oxocarbon[8]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "HydraPink", Some(&SetHighlightOpts::builder().fg(oxocarbon[14]).bg(oxocarbon[17]).build()))?;
    api::set_hl(0, "HydraHint", Some(&SetHighlightOpts::builder().fg(oxocarbon[17]).bg(oxocarbon[16]).build()))?;

    Ok("Oxocarbon Loaded".to_string())
}
