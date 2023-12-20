use chrono::Datelike;
use yew::prelude::*;
use yew_cosmo::prelude::*;

#[function_component(BambooGrovePage)]
pub fn bamboo_grove() -> Html {
    let today = chrono::Local::now();
    let year = today.year();

    html!(
        <>
            <CosmoTitle title="Bambushain Lizenz" />
            <CosmoParagraph>
                {"Bambushain ist unter der Open Source MIT Lizenz veröffentlicht, den Code kannst du dir hier anschauen: "}<CosmoAnchor href="https://github.com/Creastina/bambushain">{"Github"}</CosmoAnchor>
            </CosmoParagraph>
            <CosmoPre>
                {format!("MIT License

Copyright (c) {year} Imanuel Ulbricht, Christina Ruebsam and Hans-Jürgen Wandschneider

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.")}
            </CosmoPre>
        </>
    )
}
