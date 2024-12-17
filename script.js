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
        { id: "all_button", file: "all_wordcloud.png" },
        { id: "austria_button", file: "austria_wordcloud.png" },
        { id: "germany_button", file: "germany_wordcloud.png" },
        { id: "france_button", file: "france_wordcloud.png" },
        { id: "czechia_button", file: "czechia_wordcloud.png" }
    ];

    buttons.forEach(button => {
        fetch(button.file, { method: 'HEAD' })
            .then(response => {
                if (!response.ok) {
                    document.getElementById(button.id).style.display = 'none';
                }
            })
            .catch(() => {
                document.getElementById(button.id).style.display = 'none';
            });
    });
});

document.addEventListener("DOMContentLoaded", function() {
    const buttons = [
        { id: "all_button", file: "boxplot_multiple.html" },
        { id: "austria_button", file: "boxplot_austria.html" },
        { id: "germany_button", file: "boxplot_germany.html" },
        { id: "france_button", file: "boxplot_france.html" },
        { id: "czechia_button", file: "boxplot_czechia.html" }
    ];

    buttons.forEach(button => {
        fetch(button.file, { method: 'HEAD' })
            .then(response => {
                if (!response.ok) {
                    document.getElementById(button.id).style.display = 'none';
                }
            })
            .catch(() => {
                document.getElementById(button.id).style.display = 'none';
            });
    });
});