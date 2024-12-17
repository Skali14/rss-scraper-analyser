document.addEventListener('DOMContentLoaded', function() {
    showContent(['all_wordcloud', 'boxplot_multiple']);
    document.querySelector('.nav-button[data-content="all"]').classList.add('active');
});

function showContent(contentIds) {
    const contents = document.querySelectorAll('#content-container iframe, #content-container img');
    contents.forEach(content => content.style.display = 'none');
    contentIds.forEach(id => {
        const element = document.getElementById(id);
        element.style.display = 'block';
        if (element.tagName === 'IFRAME' && !element.src) {
            element.src = element.getAttribute('data-src');
        }
    });

    const buttons = document.querySelectorAll('.nav-button');
    buttons.forEach(button => button.classList.remove('active'));

    if (event && event.target) {
        event.target.classList.add('active');
    }
}

document.addEventListener("DOMContentLoaded", function() {
    const buttons = [
        { id: "all_button", file: "output/all_wordcloud.png" },
        { id: "austria_button", file: "output/austria_wordcloud.png" },
        { id: "germany_button", file: "output/germany_wordcloud.png" },
        { id: "france_button", file: "output/france_wordcloud.png" },
        { id: "czechia_button", file: "output/czechia_wordcloud.png" },
        { id: "gb_button", file: "output/gb_wordcloud.png" },
        { id: "all_button", file: "output/boxplot_multiple.html" },
        { id: "austria_button", file: "output/boxplot_austria.html" },
        { id: "germany_button", file: "output/boxplot_germany.html" },
        { id: "france_button", file: "output/boxplot_france.html" },
        { id: "czechia_button", file: "output/boxplot_czechia.html" },
        { id: "gb_button", file: "output/boxplot_gb.html" }
    ];

    buttons.forEach(button => {
        const element = document.getElementById(button.id);
        if (element) {
            element.style.display = 'block';
        }

        if (button.file.endsWith('.png')) {
            const img = new Image();
            img.onload = function() {
                console.log("test2")
            };
            img.onerror = function() {
                console.log("test3")
                if (element) {
                    element.style.display = 'none';
                }
            };
            img.src = button.file;
        }});
});