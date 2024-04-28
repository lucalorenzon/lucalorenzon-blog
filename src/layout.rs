use leptos::*;


use crate::components::headers::headers::BlogHeader;
use crate::components::footers::footer::BlogFooter;

#[component]
pub fn Layout( children: Children ) -> impl IntoView {

    if let [article_title, article_abstract, article_content] = &(children().nodes)[..] {
        view! {
            <BlogHeader  />
            <main class="bg-cover bg-[url('/assets/images/ostia_sea_top_image.webp')] bg-fixed h-screen z-0 -mt-[80px] overflow-auto grow-1">
                <article class="dark:bg-black/70 bg-white/70 mt-[60vh]" >
                {article_title}
                {article_abstract}
                {article_content}
                </article>
            </main>
            <BlogFooter />
        }
    }
    else {
        view! {
            <BlogHeader />
            <main class="h-screen z-0 overflow-auto grow-1">
                <h1 class="text-center md:text-5xl text-3xl px-5 font-bold">Article not correctly configured</h1>
            </main>
            <BlogFooter />


        }
    }
}

#[component]
pub fn ArticleTitle(children: Children) -> impl IntoView {
    view! {
        <h1 class="text-center md:text-5xl text-3xl px-5 font-bold">{children()}</h1>
    }
}

#[component]
pub fn ArticleAbstract(children: Children) -> impl IntoView {
    view! {
        <blockquote class="text-center md:text-3xl text-lg line-clamp-3 italic px-5 py-10 font-light font-serif">{children()}</blockquote>
    }
}

#[component]
pub fn ArticleContent(children: Children) -> impl IntoView {
    view! {
        <section class="text-left md:text-2xl text-base px-5 font-normal font-sans">{children()}</section>
    }
}