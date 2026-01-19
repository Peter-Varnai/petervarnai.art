
// Tab switching logic

const tabsAndSections = {
    'addProjectTab': [
        document.getElementById('addProjectTab'),
        document.getElementById('addProjectSection')],
    'editProjectTab': [
        document.getElementById('editProjectTab'),
        document.getElementById('editProjectSection')],
    'deleteProjectTab': [
        document.getElementById('deleteProjectTab'),
        document.getElementById('deleteProjectSection')],
    'addExhibitionTab': [
        document.getElementById('addExhibitionTab'),
        document.getElementById('addExhibitionSection')],
    'deleteExhibitionTab': [
        document.getElementById('deleteExhibitionTab'),
        document.getElementById('deleteExhibitionSection')],
}


document.querySelectorAll('.tab').forEach(button => {
    button.addEventListener('click', () => {
        handleTabClick(button.id);
    });
});


function handleTabClick(activeTab) {
    Object.entries(tabsAndSections).forEach(([key, value]) => {
        if (key === activeTab) {
            value[0].classList.add('active')
            value[1].classList.remove('hidden')
        } else {
            value[0].classList.remove('active')
            value[1].classList.add('hidden')
        }
    })
}   
