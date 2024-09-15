
use chrono::{TimeZone, Utc};
use ccv_contract::models::{CopyCategory::{Files, Html, Image, Rtf, Text, Unknown}, CopyItem, CopyItemValue, FileInfo};

pub fn every_type() -> Vec<CopyItem> {
    vec![
        CopyItem {
            id: "26".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 23, 50, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 23, 50, 0).unwrap(),
            value: CopyItemValue{
                text: Some("asdh ajlksd alsjd las;kdj als;dj lasjdsadl;k j;aslkdja l;dj al".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "25".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 23, 48, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 23, 49, 0).unwrap(),
            value: CopyItemValue{
                 text: Some("asdh ajlksd".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "24".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 23, 47, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 23, 47, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("asdh ajlksd \n\r asdasdasda da sdas da dsad".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "23".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 23, 46, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 23, 46, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: Some("iVBORw0KGgoAAAANSUhEUgAAACkAAAAgCAYAAACPb1E+AAAD50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMYm3TaF6Ix90dPPb65L8JAJX/2QCo/BtMj/97xr/+C/KeOwGI626ke+mXoz7mxfkPBkDlX2n47V9j+PM/4oHanc+g3fkM+nvvpn/tN+A/EACVf4Xp8X/P8Od/xAsy/PkfEddeT33Mi/Ovdd9vfhVf8RtnAeDF340veZcXAwDOEvwrjH/9F/xLxr/+C/71/oFf+duX5uO+4Av5ki94N17873+A73scAPz9D30Vwb9C3nMn/5K8507+9V6M9/jo1+EaAF6Ml3lxnuks9917huB/nH/gr/7+DDeeAQA4S/CvENfdyL8krruRf7uz/OZX/wD3vd6787pnAM7wuu/2BgT/Ct1Lvxz/ku6lX45/m7P85ld/Fb9y7bvxca97hmeCM69D8K9QH/Pi9C//Krwg/cu/CvUxL86/xd//0FfxK9e+G1/yLi/GAwCgNo3mhXjc3cFjr08eaHr83zP+9V+Q99wJQFx3I91Lvxz1MS/Ov8njfoBP+oF/4DmceQM+7qNfh2se9wOoTaN5IR53d/DY65P/JgBU/mcDoPI/GwCV/9kAqPzPBkDlfzYAKv+zARD8G6x+8oe48Navy723bHHvLVtceOvXZfWTP8R/AgAq/0r7n/OJHH7TV/FAwx//HsMf/x6bf/dXbH/Wl/IfCIDKv8LqJ3+Iw2/6Kl6Qw2/6KrqXeBnmb/su/Ov8A9/3aT/A3/NML/5ufMm7vBgAAGrTaF6Ix90dPPb6BODCW78uwx//Hi9M/8qvwcmf/k3+7c7ym1/9Vdz5hl/IezwW7vvNr6LyrzD+5Z/yLxn/8k/597mPO8+e4cYzAGf5+7+Fyv8UZ3+Lr/jqX+M+4JrX+xje4wzAfdx59hqCf4XuZV+Rf0n3sq/Iv8mZ1+HjvuAL+ZIv+Bhe5m+/iq/4zbMAAAT/Chvv+QH8Szbe8wP49znDi7/kGe679z7gGm488w8E/wrzt30XNj/kY3hBNj/kY5i/7bvw73OWv//bs7z4S70YcIYXf8kzqE2jeSEed3fw2OuTB1r95A9x9L3fxviXfwpA97KvyMZ7fgDzt30X/k3O/hZf8dW/xn1c8eLv9oW8x2MBAM6iNo3mhXjc3cFjr0/+mwBQ+Z8NgMr/bAD8I9r49lr3MrbPAAAAAElFTkSuQmCC".to_string()),
                files: None,
                category: Image
            }
        },
        CopyItem {
            id: "22".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 22, 55, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 22, 56, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("asdh ajlksd \n asdasdasda da sdas\n da dsad".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "21".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 21, 57, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 21, 58, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("asdh ajlksd \n a\ns\ndasdasda da sdas\n da dsad".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "20".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 20, 59, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 20, 59, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("1111asdh ajlksd \n a\ns\ndasdasda da sdas\n da dsasdkasjdlk ajsdlk;ja  kldjaslkdj al;sdjal;jd;laad".to_string()),
                html: Some("<ul data-line=\"1\" class=\"code-line\" dir=\"auto\" style=\"margin-top: 0px; margin-bottom: 0.7em; position: relative; color: rgb(59, 59, 59); font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe WPC&quot;, &quot;Segoe UI&quot;, system-ui, Ubuntu, &quot;Droid Sans&quot;, sans-serif; font-size: 14px; font-style: normal; font-variant-ligatures: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: 2; text-align: start; text-indent: 0px; text-transform: none; widows: 2; word-spacing: 0px; -webkit-text-stroke-width: 0px; white-space: normal; text-decoration-thickness: initial; text-decoration-style: initial; text-decoration-color: initial;\"><li data-line=\"1\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">add settings window</li><li data-line=\"2\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">add about menu</li><li data-line=\"3\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">create diesel context</li><li data-line=\"4\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">create schema</li><li data-line=\"5\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">make it possible to search</li><li data-line=\"6\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">add preview for each item</li><li data-line=\"7\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">make possible to create custom shortcuts</li><li data-line=\"8\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">themming</li><li data-line=\"9\" class=\"code-line\" dir=\"auto\" style=\"position: relative;\">current screen position</li></ul>".to_string()),
                rtf: None,
                image: None,
                files: None,
                category: Html
            }
        },
        CopyItem {
            id: "19".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 19, 50, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 19, 51, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: None,
                files: Some(vec![
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\ccv_rust.exe".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\".to_string()),
                        file_name: Some("ccv_rust.exe".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Some long name.xml".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\".to_string()),
                        file_name: Some("Some long name.xml".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\Another long name.xml".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("Another long name.xml".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "'C:\\Users\\pansh\\Downloads\\Printer".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("Printer".to_string()),
                        is_directory: true,
                        icon_base64: Some("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".to_string())
                    }
                ]),
                category: Files
            }
        },
        CopyItem {
            id: "18".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 18, 52, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 18, 53, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: None,
                files: Some(vec![
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\ccv_rust.exe".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\Downloads\\".to_string()),
                        file_name: Some("ccv_rust.exe".to_string()),
                        is_directory: false,
                        icon_base64: None
                    },
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\Downloads\\dev".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\Downloads\\".to_string()),
                        file_name: Some("dev".to_string()),
                        is_directory: true,
                        icon_base64: None
                    },
                    FileInfo{
                        full_path: "Another long name.xml".to_string(),
                        directory_path: None,
                        file_name: Some("Another long name.xml".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "Some long name.xml".to_string(),
                        directory_path: None,
                        file_name: Some("Some long name.xml".to_string()),
                        is_directory: false,
                        icon_base64: None
                    }
                ]),
                category: Files
            }
        },
        CopyItem {
            id: "17".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 17, 54, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 17, 55, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("        const html = item.content.find((x) => x.type === 'html') as HtmlContent;\r\n        if (html) {\r\n            return (\r\n                <div className={styles.content} dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html.html) }} />\r\n            );\r\n        }".to_string()),
                html: Some("<div style=\"color: #3b3b3b;background-color: #ffffff;font-family: Consolas, 'Courier New', monospace;font-weight: normal;font-size: 14px;line-height: 19px;white-space: pre;\"><div><span style=\"color: #3b3b3b;\">&#160; &#160; &#160; &#160; </span><span style=\"color: #0000ff;\">const</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #0070c1;\">html</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #000000;\">=</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #001080;\">item</span><span style=\"color: #3b3b3b;\">.</span><span style=\"color: #001080;\">content</span><span style=\"color: #3b3b3b;\">.</span><span style=\"color: #795e26;\">find</span><span style=\"color: #3b3b3b;\">((</span><span style=\"color: #001080;\">x</span><span style=\"color: #3b3b3b;\">) </span><span style=\"color: #0000ff;\">=&gt;</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #001080;\">x</span><span style=\"color: #3b3b3b;\">.</span><span style=\"color: #001080;\">type</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #000000;\">===</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #a31515;\">'html'</span><span style=\"color: #3b3b3b;\">) </span><span style=\"color: #af00db;\">as</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #267f99;\">HtmlContent</span><span style=\"color: #3b3b3b;\">;</span></div><div><span style=\"color: #3b3b3b;\">&#160; &#160; &#160; &#160; </span><span style=\"color: #af00db;\">if</span><span style=\"color: #3b3b3b;\"> (</span><span style=\"color: #0070c1;\">html</span><span style=\"color: #3b3b3b;\">) {</span></div><div><span style=\"color: #3b3b3b;\">&#160; &#160; &#160; &#160; &#160; &#160; </span><span style=\"color: #af00db;\">return</span><span style=\"color: #3b3b3b;\"> (</span></div><div><span style=\"color: #3b3b3b;\">&#160; &#160; &#160; &#160; &#160; &#160; &#160; &#160; </span><span style=\"color: #800000;\">&lt;</span><span style=\"color: #800000;\">div</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #e50000;\">className</span><span style=\"color: #000000;\">=</span><span style=\"color: #0000ff;\">{</span><span style=\"color: #0070c1;\">styles</span><span style=\"color: #000000;\">.</span><span style=\"color: #001080;\">content</span><span style=\"color: #0000ff;\">}</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #e50000;\">dangerouslySetInnerHTML</span><span style=\"color: #000000;\">=</span><span style=\"color: #0000ff;\">{</span><span style=\"color: #000000;\">{ </span><span style=\"color: #001080;\">__html</span><span style=\"color: #001080;\">:</span><span style=\"color: #000000;\"> </span><span style=\"color: #0070c1;\">DOMPurify</span><span style=\"color: #000000;\">.</span><span style=\"color: #795e26;\">sanitize</span><span style=\"color: #000000;\">(</span><span style=\"color: #0070c1;\">html</span><span style=\"color: #000000;\">.</span><span style=\"color: #001080;\">html</span><span style=\"color: #000000;\">) }</span><span style=\"color: #0000ff;\">}</span><span style=\"color: #3b3b3b;\"> </span><span style=\"color: #800000;\">/&gt;</span></div><div><span style=\"color: #3b3b3b;\">&#160; &#160; &#160; &#160; &#160; &#160; );</span></div><div><span style=\"color: #3b3b3b;\">&#160; &#160; &#160; &#160; }</span></div></div>".to_string()),
                rtf: None,
                image: None,
                files: None,
                category: Html
            }
        },
        CopyItem {
            id: "16".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 16, 56, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 16, 57, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("LSH JKDHBJSKLHD JKSDH KJSLHD LSDH :SHDJH SL:DJKLSJDLK JSL:DJ :LKSJD LK:JD :LKJ DL:KSJD JSLD".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "15".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 15, 58, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 15, 59, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("LSH JKDHBJSKLHD JKSDH\r\n KJSLHD LSDH :SHDJH SL:DJKLSJDLK JSL:DJ :LKSJD LK:JD :LKJ DL:KSJD JSLD".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "14".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 14, 50, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 14, 51, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: None,
                files: Some(vec![
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\ccv_rust.exe".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("ccv_rust.exe".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\ccv.exe".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("ccv.exe".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "C:\\Users\\pansh\\Downloads\\Another long name.xml".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("Another long name.xml".to_string()),
                        is_directory: false,
                        icon_base64: Some("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".to_string())
                    },
                    FileInfo{
                        full_path: "'C:\\Users\\pansh\\Downloads\\Printer".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("Printer".to_string()),
                        is_directory: true,
                        icon_base64: Some("iVBORw0KGgoAAAANSUhEUgAAAQAAAADMCAMAAACr8Y0vAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAC4lBMVEUAAAD/oQD/nwD/oAD/tRD/thX/uxX/uBf/uRX/uxf/yij/ySb/zSP/zSL/zyD/yyj/zyj/yif/yyf/yiX/oAD/nwD/nwD/oQD/oQD/oAD/oAD/nwD/oQD/oQD/oQD/oAD/oAD/oAD/nwD/oAD/oAD/oQD/oAD/oQD/oQD/nwD/oAD/nwD/oAD/oAD/oAD/nwD/oAD/oAD/nwD/nwD/nwD/nwD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/nwD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/ogL/pAT/tRP/uRf/uxX/uxX/nwD/shD/uhj/vRv/wBz/yCb/wiH/vx3/uxj/thL/wyL/wR//vhv/wB3/vx7/uhn/uhn/wSD/vhz/wyL/uxr/xiX/vh3/wiD/uRj/uBf/xCL/vhz/vBv/vBv/vx7/wiD/uBT/wB3/txb/yCb/vRz/txb/uxb/vxv/uBP/vBn/uxj/wR//uhj/vRz/txP/uRf/uRf/vh7/uBT/uRb/vRv/vRv/xSP/vBf/uxf/vBr/vBr/wiD/wiD/oQH/qAf/txT/sRD/uBf/sxL/uBf/uBf/uBj/uxv/uxv/vx//ySj/ySb/yyj/yyj/yyj/yiX/yyj/yiX/ySj/ySj/yij/yij/yij/ySb/zSP/yif/zSL/zST/yij/yij/yyj/yyj/yyj/yij/yyj/yij/ySf/zSj/zSj/zCT/zCf/ySb/yyT/ySf/yif/yif/yij/yij/yyf/yij/yij/yyb/yib/ySf/yif/yiX/yif/zCX/zSP/ySj/ySj/yib/yib/yij/ySj/yij/oAD/oQH/ogL/owP/pAT/pgX/qwr/rw7/qQj/shH/vBv/wSD/yij/pgb/tBP/vx7/tRT/xST/sQ//uxn/ogH/rAv/xCL/rg3/yCb/rAr/qAf/vRz/pwb/uBj/pwX/thT/qwn/tRNHx03iAAAA1HRSTlMAAAAAAAAAAAAAAAAAAAAAAAAAAAEFNWOFv54/AQdWpvDHAQMah+IKeUAFPNPGCQILxUELAweYXMQsQrYETMNDSqkZZ8JEp+MGCMFFV8FIEA0IAYDVkoBZJR4cEgXy4ZxDIBUB660rFvyuKhEB+IsjAdctBkkP/ncRDWAHRAHiGIwIQQHNCD6+AfgOVrwB7AGCjjOgcaR2tJbAv6iAV0lDQTMIBgHjAadnWRnxFwOpqAFKB8ZMB7YBLB4DXAMHmAsLCXprCAU80wQBCgMaiOIFNWSGwL2FxxUAAAABYktHRACIBR1IAAAAB3RJTUUH5QUDCS8L2hfaZQAAA+xJREFUeNrt1vdfTXEcx/FP183o5iKZGblm9ih7772yN9lky9577y0zXSQq2TJCiTK/98qIk0SSze/uxcNo8IN7c+79vF9/wX0/H59z7iFKlo2NY67cefLm++fyOynI4lJkKFCwUGFnYZKKOGksbb9N0WLFSwiTZWkCCk3JUs7ClBVxsSQBm9JlygoTV86CBDKULyVMXzmXCpayv2JhIRgLmGu/EJUqW4KAooq59hsFXOW/362qEIwFFK7VhDmrLncBTQ1nYV6Bmq7yfgBqCWFugdpyPoA6QnAWULhVMj+AqFtPtgKa+kJwFlC4NhDpJJBLngANhWAtoGiUXgCicRM5CmiaCt4Cjs1Eego0lx1ACyF4C1ROVwDRspXcBOoJ5gKtU/+dOrPVpm279v+hDh3TAHBPOV5/L/r+g4dmq1PnLv+hrt269+jZS/l3AH30o5jHTyQrrHefvv36D/D4M4A+9mmcZMUNHDR4iEfaAPrYZ/GSlTd02PARI9MAiH5u9fONeY4aPSY1AF3sC4lJY8eNV6YASHiZKLHJc8JE22QACa+SJEZ5TZps+xuAjtd+o8CUjL8A6F4z228QmDpN+RPgTaLELs/pM34A3HsrMWzmrEzfAXQxEstmz1F+A3gXzxNg6Nx5XwF07yWmzV+gNAJ8SOIK4LUwsxHgo8S2RYsNAPpEvgBLlmYk908S45ZlIffPnAGWryD3OM4AK1fRas77Ja85tIY1gLSW1vEGWE8beANspE28ATbTFt4AW2grb4CtAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAcALYxhtgG23nDbAdAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADkDeDNG2AH7eQNsIt28QbYTXt4A+wln32c9+/zIV8tZwCtL9nt5wxwQEX2Bxnv9zuUldT+h/kCHPFXEwUE8gUIDCCibEFHue4/GpTNAEABwVwBgo0HYDiBY8d57j9+7OsBEJ046cfyL+CU/bf9lP30GY4AZ86qvwOQKuQcv/3nQlT0oxzn2X0Qa8/noF+yu8BMQHvBjoixQIr9BoGLoXze/5cup9hP5HAlLJzH/vCwq6nsJ8oZcS2UwQeBX+i1iJyUeg7XI6NuWPf8G1GR1x0ozdT2PiHeWqs9Az+td4iPvZr+mErle/PW7Tt3ra47t2/d9FWpku/9Ai5XQwPwpZwYAAAAJXRFWHRkYXRlOmNyZWF0ZQAyMDIxLTA1LTAzVDA5OjQ3OjExKzAwOjAws7tSQwAAACV0RVh0ZGF0ZTptb2RpZnkAMjAyMS0wNS0wM1QwOTo0NzoxMSswMDowMMLm6v8AAAAASUVORK5CYII=".to_string())
                    },
                ]),
                category: Files
            }
        },
        CopyItem {
            id: "13".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 13, 52, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 13, 53, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: None,
                files: Some(vec![
                    FileInfo{
                        full_path: "'C:\\Users\\pansh\\Downloads\\Printer".to_string(),
                        directory_path: Some("C:\\Users\\pansh\\Downloads\\".to_string()),
                        file_name: Some("Printer".to_string()),
                        is_directory: true,
                        icon_base64: Some("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".to_string())
                    },
                ]),
                category: Files
            }
        },
        CopyItem {
            id: "12".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 12, 54, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 12, 54, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("LSH JKDH\nBJSKLHD JKSDH\n KJSLHD LSDH :SHDJH \nSL:DJKLSJDLK JSL:DJ :LKSJD LK:JD :LKJ DL:KSJD JSLD".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "11".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 11, 55, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 11, 55, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("L\nSH JKDH\nBJSKLHD JKSDH\n KJSLHD LasdadSDH :SHDJH \nSL:DJKLSJDLK JSL:DJ :LKSJD LK:JD :LKJ DL:KSJD JSLD".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "10".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 10, 56, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 10, 56, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: Some("iVBORw0KGgoAAAANSUhEUgAAACkAAAAgCAYAAACPb1E+AAAD50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMYm3TaF6Ix90dPPb65L8JAJX/2QCo/BtMj/97xr/+C/KeOwGI626ke+mXoz7mxfkPBkDlX2n47V9j+PM/4oHanc+g3fkM+nvvpn/tN+A/EACVf4Xp8X/P8Od/xAsy/PkfEddeT33Mi/Ovdd9vfhVf8RtnAeDF340veZcXAwDOEvwrjH/9F/xLxr/+C/71/oFf+duX5uO+4Av5ki94N17873+A73scAPz9D30Vwb9C3nMn/5K8507+9V6M9/jo1+EaAF6Ml3lxnuks9917huB/nH/gr/7+DDeeAQA4S/CvENfdyL8krruRf7uz/OZX/wD3vd6787pnAM7wuu/2BgT/Ct1Lvxz/ku6lX45/m7P85ld/Fb9y7bvxca97hmeCM69D8K9QH/Pi9C//Krwg/cu/CvUxL86/xd//0FfxK9e+G1/yLi/GAwCgNo3mhXjc3cFjr08eaHr83zP+9V+Q99wJQFx3I91Lvxz1MS/Ov8njfoBP+oF/4DmceQM+7qNfh2se9wOoTaN5IR53d/DY65P/JgBU/mcDoPI/GwCV/9kAqPzPBkDlfzYAKv+zARD8G6x+8oe48Navy723bHHvLVtceOvXZfWTP8R/AgAq/0r7n/OJHH7TV/FAwx//HsMf/x6bf/dXbH/Wl/IfCIDKv8LqJ3+Iw2/6Kl6Qw2/6KrqXeBnmb/su/Ov8A9/3aT/A3/NML/5ufMm7vBgAAGrTaF6Ix90dPPb6BODCW78uwx//Hi9M/8qvwcmf/k3+7c7ym1/9Vdz5hl/IezwW7vvNr6LyrzD+5Z/yLxn/8k/597mPO8+e4cYzAGf5+7+Fyv8UZ3+Lr/jqX+M+4JrX+xje4wzAfdx59hqCf4XuZV+Rf0n3sq/Iv8mZ1+HjvuAL+ZIv+Bhe5m+/iq/4zbMAAAT/Chvv+QH8Szbe8wP49znDi7/kGe679z7gGm488w8E/wrzt30XNj/kY3hBNj/kY5i/7bvw73OWv//bs7z4S70YcIYXf8kzqE2jeSEed3fw2OuTB1r95A9x9L3fxviXfwpA97KvyMZ7fgDzt30X/k3O/hZf8dW/xn1c8eLv9oW8x2MBAM6iNo3mhXjc3cFjr0/+mwBQ+Z8NgMr/bAD8I9r49lr3MrbPAAAAAElFTkSuQmCC".to_string()),
                files: None,
                category: Image
            }
        },
        CopyItem {
            id: "9".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 9, 57, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 9, 57, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: Some("iVBORw0KGgoAAAANSUhEUgAAACkAAAAgCAYAAACPb1E+AAAD50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMYm3TaF6Ix90dPPb65L8JAJX/2QCo/BtMj/97xr/+C/KeOwGI626ke+mXoz7mxfkPBkDlX2n47V9j+PM/4oHanc+g3fkM+nvvpn/tN+A/EACVf4Xp8X/P8Od/xAsy/PkfEddeT33Mi/Ovdd9vfhVf8RtnAeDF340veZcXAwDOEvwrjH/9F/xLxr/+C/71/oFf+duX5uO+4Av5ki94N17873+A73scAPz9D30Vwb9C3nMn/5K8507+9V6M9/jo1+EaAF6Ml3lxnuks9917huB/nH/gr/7+DDeeAQA4S/CvENfdyL8krruRf7uz/OZX/wD3vd6787pnAM7wuu/2BgT/Ct1Lvxz/ku6lX45/m7P85ld/Fb9y7bvxca97hmeCM69D8K9QH/Pi9C//Krwg/cu/CvUxL86/xd//0FfxK9e+G1/yLi/GAwCgNo3mhXjc3cFjr08eaHr83zP+9V+Q99wJQFx3I91Lvxz1MS/Ov8njfoBP+oF/4DmceQM+7qNfh2se9wOoTaN5IR53d/DY65P/JgBU/mcDoPI/GwCV/9kAqPzPBkDlfzYAKv+zARD8G6x+8oe48Navy723bHHvLVtceOvXZfWTP8R/AgAq/0r7n/OJHH7TV/FAwx//HsMf/x6bf/dXbH/Wl/IfCIDKv8LqJ3+Iw2/6Kl6Qw2/6KrqXeBnmb/su/Ov8A9/3aT/A3/NML/5ufMm7vBgAAGrTaF6Ix90dPPb6BODCW78uwx//Hi9M/8qvwcmf/k3+7c7ym1/9Vdz5hl/IezwW7vvNr6LyrzD+5Z/yLxn/8k/597mPO8+e4cYzAGf5+7+Fyv8UZ3+Lr/jqX+M+4JrX+xje4wzAfdx59hqCf4XuZV+Rf0n3sq/Iv8mZ1+HjvuAL+ZIv+Bhe5m+/iq/4zbMAAAT/Chvv+QH8Szbe8wP49znDi7/kGe679z7gGm488w8E/wrzt30XNj/kY3hBNj/kY5i/7bvw73OWv//bs7z4S70YcIYXf8kzqE2jeSEed3fw2OuTB1r95A9x9L3fxviXfwpA97KvyMZ7fgDzt30X/k3O/hZf8dW/xn1c8eLv9oW8x2MBAM6iNo3mhXjc3cFjr0/+mwBQ+Z8NgMr/bAD8I9r49lr3MrbPAAAAAElFTkSuQmCC".to_string()),
                files: None,
                category: Image
            }
        },
        CopyItem {
            id: "8".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 8, 58, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 8, 58, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("This is some bold text".to_string()),
                html: None,
                rtf: Some("{\\rtf1\\ansi{\\fonttbl\\f0\\fswiss Helvetica;}\\f0\\pard\r\nThis is some {\\b bold} text.\\par\r\n }".to_string()),
                image: None,
                files: None,
                category: Rtf
            }
        },
        CopyItem {
            id: "7".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 7, 59, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 7, 59, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("asdh ajlksd \n a\ns\ndasdasda da sdas\n da dsad".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "6".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 6, 50, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 6, 50, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("asdh ajlksd \n a\ns\ndasdasda da sdas\n da dsasdkasjdlk ajsdlk;ja  kldjaslkdj al;sdjal;jd;laad".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "5".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 5, 51, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 5, 51, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: Some("iVBORw0KGgoAAAANSUhEUgAAAQAAAADMCAMAAACr8Y0vAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAC4lBMVEUAAAD/oQD/nwD/oAD/tRD/thX/uxX/uBf/uRX/uxf/yij/ySb/zSP/zSL/zyD/yyj/zyj/yif/yyf/yiX/oAD/nwD/nwD/oQD/oQD/oAD/oAD/nwD/oQD/oQD/oQD/oAD/oAD/oAD/nwD/oAD/oAD/oQD/oAD/oQD/oQD/nwD/oAD/nwD/oAD/oAD/oAD/nwD/oAD/oAD/nwD/nwD/nwD/nwD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/nwD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/oAD/nwD/nwD/ogL/pAT/tRP/uRf/uxX/uxX/nwD/shD/uhj/vRv/wBz/yCb/wiH/vx3/uxj/thL/wyL/wR//vhv/wB3/vx7/uhn/uhn/wSD/vhz/wyL/uxr/xiX/vh3/wiD/uRj/uBf/xCL/vhz/vBv/vBv/vx7/wiD/uBT/wB3/txb/yCb/vRz/txb/uxb/vxv/uBP/vBn/uxj/wR//uhj/vRz/txP/uRf/uRf/vh7/uBT/uRb/vRv/vRv/xSP/vBf/uxf/vBr/vBr/wiD/wiD/oQH/qAf/txT/sRD/uBf/sxL/uBf/uBf/uBj/uxv/uxv/vx//ySj/ySb/yyj/yyj/yyj/yiX/yyj/yiX/ySj/ySj/yij/yij/yij/ySb/zSP/yif/zSL/zST/yij/yij/yyj/yyj/yyj/yij/yyj/yij/ySf/zSj/zSj/zCT/zCf/ySb/yyT/ySf/yif/yif/yij/yij/yyf/yij/yij/yyb/yib/ySf/yif/yiX/yif/zCX/zSP/ySj/ySj/yib/yib/yij/ySj/yij/oAD/oQH/ogL/owP/pAT/pgX/qwr/rw7/qQj/shH/vBv/wSD/yij/pgb/tBP/vx7/tRT/xST/sQ//uxn/ogH/rAv/xCL/rg3/yCb/rAr/qAf/vRz/pwb/uBj/pwX/thT/qwn/tRNHx03iAAAA1HRSTlMAAAAAAAAAAAAAAAAAAAAAAAAAAAEFNWOFv54/AQdWpvDHAQMah+IKeUAFPNPGCQILxUELAweYXMQsQrYETMNDSqkZZ8JEp+MGCMFFV8FIEA0IAYDVkoBZJR4cEgXy4ZxDIBUB660rFvyuKhEB+IsjAdctBkkP/ncRDWAHRAHiGIwIQQHNCD6+AfgOVrwB7AGCjjOgcaR2tJbAv6iAV0lDQTMIBgHjAadnWRnxFwOpqAFKB8ZMB7YBLB4DXAMHmAsLCXprCAU80wQBCgMaiOIFNWSGwL2FxxUAAAABYktHRACIBR1IAAAAB3RJTUUH5QUDCS8L2hfaZQAAA+xJREFUeNrt1vdfTXEcx/FP183o5iKZGblm9ih7772yN9lky9577y0zXSQq2TJCiTK/98qIk0SSze/uxcNo8IN7c+79vF9/wX0/H59z7iFKlo2NY67cefLm++fyOynI4lJkKFCwUGFnYZKKOGksbb9N0WLFSwiTZWkCCk3JUs7ClBVxsSQBm9JlygoTV86CBDKULyVMXzmXCpayv2JhIRgLmGu/EJUqW4KAooq59hsFXOW/362qEIwFFK7VhDmrLncBTQ1nYV6Bmq7yfgBqCWFugdpyPoA6QnAWULhVMj+AqFtPtgKa+kJwFlC4NhDpJJBLngANhWAtoGiUXgCicRM5CmiaCt4Cjs1Eego0lx1ACyF4C1ROVwDRspXcBOoJ5gKtU/+dOrPVpm279v+hDh3TAHBPOV5/L/r+g4dmq1PnLv+hrt269+jZS/l3AH30o5jHTyQrrHefvv36D/D4M4A+9mmcZMUNHDR4iEfaAPrYZ/GSlTd02PARI9MAiH5u9fONeY4aPSY1AF3sC4lJY8eNV6YASHiZKLHJc8JE22QACa+SJEZ5TZps+xuAjtd+o8CUjL8A6F4z228QmDpN+RPgTaLELs/pM34A3HsrMWzmrEzfAXQxEstmz1F+A3gXzxNg6Nx5XwF07yWmzV+gNAJ8SOIK4LUwsxHgo8S2RYsNAPpEvgBLlmYk908S45ZlIffPnAGWryD3OM4AK1fRas77Ja85tIY1gLSW1vEGWE8beANspE28ATbTFt4AW2grb4CtAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAcALYxhtgG23nDbAdAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADkDeDNG2AH7eQNsIt28QbYTXt4A+wln32c9+/zIV8tZwCtL9nt5wxwQEX2Bxnv9zuUldT+h/kCHPFXEwUE8gUIDCCibEFHue4/GpTNAEABwVwBgo0HYDiBY8d57j9+7OsBEJ046cfyL+CU/bf9lP30GY4AZ86qvwOQKuQcv/3nQlT0oxzn2X0Qa8/noF+yu8BMQHvBjoixQIr9BoGLoXze/5cup9hP5HAlLJzH/vCwq6nsJ8oZcS2UwQeBX+i1iJyUeg7XI6NuWPf8G1GR1x0ozdT2PiHeWqs9Az+td4iPvZr+mErle/PW7Tt3ra47t2/d9FWpku/9Ai5XQwPwpZwYAAAAJXRFWHRkYXRlOmNyZWF0ZQAyMDIxLTA1LTAzVDA5OjQ3OjExKzAwOjAws7tSQwAAACV0RVh0ZGF0ZTptb2RpZnkAMjAyMS0wNS0wM1QwOTo0NzoxMSswMDowMMLm6v8AAAAASUVORK5CYII=".to_string()),
                files: None,
                category: Image
            }
        },
        CopyItem {
            id: "4".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 4, 52, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 4, 52, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("asdh ajlksd \n a\ns\ndasdasda da sdas\n da dsasdkasjdlk ajsdlk;ja  kldjaslkdj al;sdjal;jd;laad".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "3".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 3, 53, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 3, 53, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Unknown
            }
        },
        CopyItem {
            id: "2".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 2, 54, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 2, 54, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("LSH JKDHBJSKLHD JKSDH KJSLHD LSDH :SHDJH SL:DJKLSJDLK JSL:DJ :LKSJD LK:JD :LKJ DL:KSJD JSLD".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
        CopyItem {
            id: "1".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 1, 55, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 1, 55, 0).unwrap(),
            value: CopyItemValue{ 
                text: None,
                html: None,
                rtf: None,
                image: Some("iVBORw0KGgoAAAANSUhEUgAAACkAAAAgCAYAAACPb1E+AAAD50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMYm3TaF6Ix90dPPb65L8JAJX/2QCo/BtMj/97xr/+C/KeOwGI626ke+mXoz7mxfkPBkDlX2n47V9j+PM/4oHanc+g3fkM+nvvpn/tN+A/EACVf4Xp8X/P8Od/xAsy/PkfEddeT33Mi/Ovdd9vfhVf8RtnAeDF340veZcXAwDOEvwrjH/9F/xLxr/+C/71/oFf+duX5uO+4Av5ki94N17873+A73scAPz9D30Vwb9C3nMn/5K8507+9V6M9/jo1+EaAF6Ml3lxnuks9917huB/nH/gr/7+DDeeAQA4S/CvENfdyL8krruRf7uz/OZX/wD3vd6787pnAM7wuu/2BgT/Ct1Lvxz/ku6lX45/m7P85ld/Fb9y7bvxca97hmeCM69D8K9QH/Pi9C//Krwg/cu/CvUxL86/xd//0FfxK9e+G1/yLi/GAwCgNo3mhXjc3cFjr08eaHr83zP+9V+Q99wJQFx3I91Lvxz1MS/Ov8njfoBP+oF/4DmceQM+7qNfh2se9wOoTaN5IR53d/DY65P/JgBU/mcDoPI/GwCV/9kAqPzPBkDlfzYAKv+zARD8G6x+8oe48Navy723bHHvLVtceOvXZfWTP8R/AgAq/0r7n/OJHH7TV/FAwx//HsMf/x6bf/dXbH/Wl/IfCIDKv8LqJ3+Iw2/6Kl6Qw2/6KrqXeBnmb/su/Ov8A9/3aT/A3/NML/5ufMm7vBgAAGrTaF6Ix90dPPb6BODCW78uwx//Hi9M/8qvwcmf/k3+7c7ym1/9Vdz5hl/IezwW7vvNr6LyrzD+5Z/yLxn/8k/597mPO8+e4cYzAGf5+7+Fyv8UZ3+Lr/jqX+M+4JrX+xje4wzAfdx59hqCf4XuZV+Rf0n3sq/Iv8mZ1+HjvuAL+ZIv+Bhe5m+/iq/4zbMAAAT/Chvv+QH8Szbe8wP49znDi7/kGe679z7gGm488w8E/wrzt30XNj/kY3hBNj/kY5i/7bvw73OWv//bs7z4S70YcIYXf8kzqE2jeSEed3fw2OuTB1r95A9x9L3fxviXfwpA97KvyMZ7fgDzt30X/k3O/hZf8dW/xn1c8eLv9oW8x2MBAM6iNo3mhXjc3cFjr0/+mwBQ+Z8NgMr/bAD8I9r49lr3MrbPAAAAAElFTkSuQmCC".to_string()),
                files: None,
                category: Image
            }
        },
        CopyItem {
            id: "0".to_string(),
            created:  Utc.with_ymd_and_hms(2021, 8, 23, 0, 56, 0).unwrap(),
            last_reuse: Utc.with_ymd_and_hms(2021, 8, 23, 0, 56, 0).unwrap(),
            value: CopyItemValue{ 
                text: Some("LSH JKDHBJSKLHD JKSDH\n KJSLHD LSDH :SHDJH SL:DJKLSJDLK JSL:DJ :LKSJD LK:JD :LKJ DL:KSJD JSLD".to_string()),
                html: None,
                rtf: None,
                image: None,
                files: None,
                category: Text
            }
        },
    ]
}
