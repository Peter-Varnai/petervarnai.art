const smileyArr = [
    [':)))))))', '::))))))', ':::)))))', '::::))))', ':::::)))', '::::::))', ':::::::)', '::::::))', ':::::)))', '::::))))', ':::)))))', '::))))))', ':)))))))'],
    [';)))))))', ';;))))))', ';;;)))))', ';;;;))))', ';;;;;)))', ';;;;;;))', ';;;;;;;)', ';;;;;;))', ';;;;;)))', ';;;;))))', ';;;)))))', ';;))))))', ';)))))))']
];

let loadRound = 0;
let loadType = false;
let animationId = null;
let lastUpdateTime = 0;
const frameInterval = 90; // Match original setTimeout timing

const ls = document.getElementById('loadingScreen');

function loadingAnim(timestamp) {
    if (timestamp - lastUpdateTime >= frameInterval) {
        if (loadType) {
            ls.innerText = smileyArr[0][loadRound];
        } else {
            ls.innerText = smileyArr[1][loadRound];
        }
        
        loadRound++;
        if (loadRound > 12) {
            loadType = !loadType;
            loadRound = 0;
        }
        
        lastUpdateTime = timestamp;
    }
    
    animationId = requestAnimationFrame(loadingAnim);
}

function startLoadingAnim() {
    lastUpdateTime = performance.now();
    loadingAnim(lastUpdateTime);
}

function stopLoadingAnim() {
    if (animationId !== null) {
        cancelAnimationFrame(animationId);
        animationId = null;
    }
    ls.style.display = 'none';
}

export { startLoadingAnim, stopLoadingAnim };