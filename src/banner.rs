use anstream::println;
use anstyle::{AnsiColor, Color, RgbColor, Style};

use crate::prefix;
use crate::VERSION;

const BANNER: [&str; 6] = [
    "██████╗ ███████╗██╗   ██╗███████╗ ██████╗ ██████╗  ██████╗ ███████╗",
    "██╔══██╗██╔════╝██║   ██║██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝",
    "██║  ██║█████╗  ██║   ██║█████╗  ██║   ██║██████╔╝██║  ███╗█████╗",
    "██║  ██║██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝",
    "██████╔╝███████╗ ╚████╔╝ ██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗",
    "╚═════╝ ╚══════╝  ╚═══╝  ╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝",
];

const BANNER_STYLES: [Style; 6] = [
    Style::new().fg_color(Some(Color::Rgb(RgbColor(45, 212, 191)))),
    Style::new().fg_color(Some(Color::Rgb(RgbColor(34, 211, 238)))),
    Style::new().fg_color(Some(Color::Rgb(RgbColor(96, 165, 250)))),
    Style::new().fg_color(Some(Color::Rgb(RgbColor(129, 140, 248)))),
    Style::new().fg_color(Some(Color::Rgb(RgbColor(192, 132, 252)))),
    Style::new().fg_color(Some(Color::Rgb(RgbColor(232, 121, 249)))),
];

const TITLE: Style = Style::new()
    .fg_color(Some(Color::Ansi(AnsiColor::BrightCyan)))
    .bold();
const ACCENT: Style = Style::new()
    .fg_color(Some(Color::Ansi(AnsiColor::BrightGreen)))
    .bold();
const MUTED: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)));
const BOLD: Style = Style::new().bold();

pub fn print_home() {
    println!();
    for (line, style) in BANNER.iter().zip(BANNER_STYLES) {
        println!("  {style}{line}{style:#}");
    }

    println!();
    println!("  {TITLE}devforge{TITLE:#} {MUTED}{VERSION}{MUTED:#}");
    println!("  Generador de scaffolds de proyecto basado en convenciones de prefijos.");

    let prefix_count = prefix::all().len();
    println!("  {MUTED}{prefix_count} prefijos disponibles.{MUTED:#}");

    println!();
    println!("  {BOLD}Comandos{BOLD:#}");
    println!("    {ACCENT}devforge list{ACCENT:#}      Ver prefijos y lenguajes soportados");
    println!("    {ACCENT}devforge --help{ACCENT:#}    Ver todas las opciones");

    println!();
    println!("  {BOLD}Ejemplo{BOLD:#}");
    println!("    {MUTED}devforge create api-catalog --lang go{MUTED:#}");
    println!();
}
