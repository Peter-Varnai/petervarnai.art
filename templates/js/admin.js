
// Tab switching logic
const editTab = document.getElementById('editTab');
const addTab = document.getElementById('addTab');
const editSection = document.getElementById('editProjectSection');
const addSection = document.getElementById('addProjectSection');


editTab.addEventListener('click', function() {
    editTab.classList.add('active');
    addTab.classList.remove('active');
    editSection.classList.remove('hidden');
    addSection.classList.add('hidden');
});

addTab.addEventListener('click', function() {
    addTab.classList.add('active');
    editTab.classList.remove('active');
    addSection.classList.remove('hidden');
    editSection.classList.add('hidden');
});
