import * as THREE from 'three'
import {GLTFLoader} from 'three/addons/loaders/GLTFLoader.js'
import {OrbitControls} from 'three/addons/controls/OrbitControls.js'
import {ShaderPass, EffectComposer} from 'postprocessing';
import {AnimationMixer, DataTexture, FloatType, MathUtils, Object3D, RedFormat, ShaderMaterial} from "three";
import {CSS3DObject, CSS3DRenderer} from "three/addons";


let WIDTH = window.innerWidth
let HEIGHT = window.innerHeight
let mql = window.matchMedia('(max-width: 640px)').matches


const scene = new THREE.Scene()
const shadowScene = new THREE.Scene()


const camera = new THREE.PerspectiveCamera(mql ? 16 : 13, WIDTH / HEIGHT, 0.1, 1000)
camera.position.set(8, 21, 48)
const renderer = new THREE.WebGLRenderer()
renderer.shadowMap.enabled = true


// const controls = new OrbitControls(camera, renderer.domElement)


renderer.setSize(WIDTH, HEIGHT)
renderer.setClearColor('#000000', 1)
document.body.appendChild(renderer.domElement)


const shadowLight = new THREE.SpotLight('#ffe5d2', 1132)
const dirLight = new THREE.DirectionalLight('#ffe9dd', 2.7)
const contourLight = new THREE.SpotLight('#ffffff', 88)
const pLight = new THREE.PointLight('#fdeaea', 24)


shadowLight.castShadow = true
shadowLight.shadow.bias = -0.001
mql ? shadowLight.position.set(1, 20, -9) : shadowLight.position.set(12, 23, 22)
shadowLight.shadow.mapSize.width = 4096
shadowLight.shadow.mapSize.height = 4096
shadowScene.add(shadowLight)


dirLight.position.set(12, 22, 22)
scene.add(dirLight)


contourLight.position.set(5, 9, -6)
scene.add(contourLight)


pLight.position.set(-5, 12, 6)
scene.add(pLight)


const gltfLoader = new GLTFLoader()
let fireMixer, shadowFireMixer, kbbMixer, engineFire, kbbRocket, kbbRotationGroup, kbbRocketShadow, camTrgt,
    camTrgtMixer, engineBody, engineMixer, kbbRotate, shadowMixer, shadowEngineFire, lampsMixer, interfaceLamps,
    camTrgtMobile, engineFireShadow
const fallAnimArr = []


function loadScene() {
    return new Promise((resolve, reject) => {
        gltfLoader.load('3dFiles/kebabGLB.glb', function (glb) {
            const root = glb.scene
            scene.add(root.clone())
            shadowScene.add(root)


            const anims = glb.animations
            kbbRocket = scene.getObjectByName('kebabBody')
            const engine = scene.getObjectByName('engine')
            kbbRocketShadow = shadowScene.getObjectByName('kebabRocket')
            kbbRotationGroup = scene.getObjectByName('kebabRocket')
            engineFire = scene.getObjectByName('engine_fire')
            engineFireShadow = shadowScene.getObjectByName('engine_fire')


            // camera setup
            camTrgt = scene.getObjectByName('camTrgt')
            camTrgtMobile = scene.getObjectByName('camTrgtmobile')
            camera.lookAt(mql ? camTrgtMobile.position : camTrgt.position)


            // animation mixer setup
            kbbMixer = new AnimationMixer(kbbRocket)
            fireMixer = new AnimationMixer(engineFire)
            engineMixer = new AnimationMixer(engine)
            camTrgtMixer = new AnimationMixer(camTrgt)
            shadowMixer = new AnimationMixer(kbbRocketShadow)


            shadowFireMixer = new THREE.AnimationMixer(engineFireShadow)
            for (let i = 0; i < anims.length; i++) {
                const clip = anims[i]
                if (clip.name.includes('fire')) {
                    fireMixer.clipAction(clip).play()
                    shadowFireMixer.clipAction(clip).play()
                } else if (clip.name.includes('ec')) {
                    setupFallAction(clip, engineMixer)
                    setupFallAction(clip, shadowMixer)
                } else if (clip.name === 'camTrgtFall') {
                    setupFallAction(clip, camTrgtMixer)
                } else {
                    setupFallAction(clip, kbbMixer)
                    setupFallAction(clip, shadowMixer)
                }
            }


            // switching on shadows
            kbbRocket.traverse(function (item) {
                if (item instanceof THREE.Mesh) {
                    item.receiveShadow = true
                    item.castShadow = true
                }
            })
            const shadowKbbMat = new THREE.MeshStandardMaterial({
                opacity: 0,
                transparent: true,
            })
            kbbRocketShadow.traverse((child) => {
                if (child instanceof THREE.Mesh) {
                    child.material = shadowKbbMat
                    child.castShadow = true
                    child.receiveShadow = false
                }
            })
            shadowEngineFire = kbbRocketShadow.getObjectByName('engine_fire').children
            shadowEngineFire.forEach(child => {
                if (!child.name === 'fire2') {
                    child.visible = false
                }
            })


            // kebab texture setup
            const kbbIngredients = scene.getObjectByName('kebabBody').children
            kbbIngredients.forEach(ingredient => ingredient.name !== 'camTrgt' ? colorToBump(ingredient) : null)


            // engine material setup
            engineBody = scene.getObjectByName('staticEngineBody')
            const envTextureObject = scene.getObjectByName("Mesh076_1")
            const envTexture = envTextureObject.material.map
            const aoTexture = envTextureObject.material.clearcoatMap
            envTexture.mapping = THREE.EquirectangularReflectionMapping
            engineBody.children.forEach(child => {
                child.material.envMap = envTexture
                child.material.roughnessMap = aoTexture
                child.material.metalnessMap = aoTexture
                if (child.name === 'Mesh076') {
                    child.material.color = new THREE.Color(0.12, 0.12, 0.12)
                }
            })
            envTextureObject.material.map = null


            // TODO: this doesnt work yet
            const flameMatCol = scene.getObjectByName('fire1').material.map
            const flameMatAlpha = scene.getObjectByName('fire1').material.metalnessMap
            const flameMat = new THREE.MeshPhysicalMaterial({
                map: flameMatCol,
                color: '#000000',
                reflectivity: 0,
                alphaTest: 0.4,
                alphaMap: flameMatAlpha,
                transparent: true,
                emissiveIntensity: 2,
                emissive: '#f89d7a',
                emissiveMap: flameMatCol,
                opacity: 0.7,
                blending: THREE.CustomBlending,
                blendSrc: THREE.SrcAlphaFactor,
                blendDst: THREE.OneMinusSrcAlphaFactor,
                blendEquation: THREE.MaxEquation
            })

            engineFire.children.forEach(child => {
                child.material = flameMat
            })


            // rocket lamps setup
            // bulb lamps setup
            const upperLampsArr = scene.getObjectByName('upperLamps').children
            const upperLampMat = new THREE.MeshStandardMaterial(
                {
                    emissiveIntensity: 1,
                    emissive: '#f1d67e',
                    reflectivity: 1,
                })
            upperLampsArr.forEach(lamp => {
                lamp.material = upperLampMat
            })
            lampsMixer = new THREE.AnimationMixer(upperLampMat)
            const lampPulsKf = new THREE.KeyframeTrack(
                `${upperLampMat.uuid}.emissiveIntensity`,
                [0, 2, 3, 5],
                [0.2, 0.2, 1.6, 0.2]);
            const lampPulseClip = new THREE.AnimationClip('lampPuls', 5, [lampPulsKf]);
            lampsMixer.clipAction(lampPulseClip).play()


            // lamp array setup
            const lowerLampsAr = scene.getObjectByName('lowerLamps').children
            lowerLampsAr.forEach(lamp => {
                lamp.material = new THREE.MeshStandardMaterial(
                    {
                        color: '#000000',
                        emissiveIntensity: 1,
                        emissive: '#ffcc84',
                        reflectivity: 0,
                    })
            })
            let lampCount = 0
            let prevLamp1, prevLamp2, prevLamp3

            setInterval(arrayLampsAnimation, 50)

            function arrayLampsAnimation() {
                if (lampCount === 0) {
                    prevLamp1 = 39
                    prevLamp2 = 38
                    prevLamp3 = 37
                } else if (lampCount === 1) {
                    prevLamp1 = 0
                    prevLamp2 = 39
                    prevLamp3 = 38
                } else if (lampCount === 2) {
                    prevLamp1 = 1
                    prevLamp2 = 0
                    prevLamp3 = 39
                } else {
                    prevLamp1 = lampCount - 1
                    prevLamp2 = lampCount - 2
                    prevLamp3 = lampCount - 3
                }
                lowerLampsAr.forEach(lamp => lamp.material.emissiveIntensity = 0.1)
                lowerLampsAr[lampCount].material.emissiveIntensity = 2
                lowerLampsAr[prevLamp1].material.emissiveIntensity = 1
                lowerLampsAr[prevLamp2].material.emissiveIntensity = 0.8
                lowerLampsAr[prevLamp3].material.emissiveIntensity = 0.6
                if (lampCount === 39) {
                    lampCount = 0
                } else {
                    lampCount++
                }
            }


            // interface lamps flashing
            const lampColorArr = ['#ff2a2a', '#0004ff', '#2eff00']
            interfaceLamps = scene.getObjectByName('interfaceLamps').children
            interfaceLamps.forEach(lamp => {
                const randColorIndex = Math.floor(Math.random() * 3)
                const randCol = lampColorArr[randColorIndex]
                lamp.material = new THREE.MeshStandardMaterial({
                    emissiveIntensity: 2,
                    emissive: randCol,
                    reflectivity: 0,
                })
            })


            // ground setup
            const ground = shadowScene.getObjectByName('ground')
            scene.getObjectByName('ground').visible = false
            scene.traverse(child => {
                child.name === 'ground' ? scene.remove(child) : null
            })
            ground.material = new THREE.ShadowMaterial()
            ground.receiveShadow = true

            resolve()
        }, undefined, function (error) {
            reject(error)
        })
    })
}


function setupFallAction(inputClip, mixer) {
    const fallAction = mixer.clipAction(inputClip)
    fallAction.setLoop(THREE.LoopOnce)
    fallAction.clampWhenFinished = true
    fallAnimArr.push(fallAction)
}


const clock = new THREE.Clock()


function colorToBump(input) {
    const inputMat = input.material
    inputMat.reflectivity = 0.1
    inputMat.bumpMap = inputMat.map
    inputMat.bumpScale = 0.4
}


// POST PROCESSES
const compositRenderTarget1 = new THREE.WebGLRenderTarget(WIDTH, HEIGHT)
const compositRenderTarget2 = new THREE.WebGLRenderTarget(WIDTH, HEIGHT)


const digitalGlitch = {
    uniforms: {
        tDiffuse: {value: compositRenderTarget1.texture}, //diffuse texture
        input2: {value: compositRenderTarget2.texture},
        tDisp: {value: generateHeightmap(randFloat())}, //displacement texture for digital glitch squares
        byp: {value: 1}, //apply the glitch ?
        amount: {value: 0.0018},
        angle: {value: 0.02},
        seed: {value: 0.122},
        seed_x: {value: 0.02}, //-1,1
        seed_y: {value: 0.02}, //-1,1
        distortion_x: {value: 0.0},
        distortion_y: {value: 0.0},
        col_s: {value: 0.01}
    },
    vertexShader: `
		varying vec2 vUv;
		void main() {
			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
		}`,
    fragmentShader: `
		uniform int byp; //should we apply the glitch ?

		uniform sampler2D tDiffuse;
		uniform sampler2D input2;
		uniform sampler2D tDisp;

		uniform float amount;
		uniform float angle;
		uniform float seed;
		uniform float seed_x;
		uniform float seed_y;
		uniform float distortion_x;
		uniform float distortion_y;
		uniform float col_s;

		varying vec2 vUv;


		float rand(vec2 co){
			return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
		}

		void main() {
			if(byp<1) {
				vec2 p = vUv;
				float xs = floor(gl_FragCoord.x / 0.5);
				float ys = floor(gl_FragCoord.y / 0.5);
				
				float disp = texture2D(tDisp, p*seed*seed).r;
				if(p.y<distortion_x+col_s && p.y>distortion_x-col_s*seed) {
					if(seed_x>0.){
						p.y = 1. - (p.y + distortion_y);
					}
					else {
						p.y = distortion_y;
					}
				}
				if(p.x<distortion_y+col_s && p.x>distortion_y-col_s*seed) {
					if(seed_y>0.){
						p.x=distortion_x;
					}
					else {
						p.x = 1. - (p.x + distortion_x);
					}
				}
				p.x+=disp*seed_x*(seed/5.);
				p.y+=disp*seed_y*(seed/5.);
				
				//base from RGB shift shader
				vec2 offset = amount * vec2( cos(angle), sin(angle));
				vec4 cr = texture2D(tDiffuse, p + offset);
				vec4 cga = texture2D(tDiffuse, p);
				vec4 cb = texture2D(tDiffuse, p - offset);
				gl_FragColor = vec4(cr.r, cga.g, cb.b, cga.a);
	
			    // overlap tDiffuse to input2
                vec4 texel2 = texture2D(input2, vUv);
			    gl_FragColor = texel2 + gl_FragColor;
				
			}
			else {
			    vec4 texel2 = texture2D (input2, vUv);
				gl_FragColor=texture2D (tDiffuse, vUv);
				gl_FragColor=gl_FragColor + texel2;
				// gl_FragColor=texel2;
			}
		}`
}


function generateHeightmap(dt_size) {
    const data_arr = new Float32Array(dt_size * dt_size)
    const length = dt_size * dt_size

    for (let i = 0; i < length; i++) {
        const val = MathUtils.randFloat(0, 1)
        data_arr[i] = val
    }

    const texture = new DataTexture(data_arr, dt_size, dt_size, RedFormat, FloatType)
    texture.needsUpdate = true
    return texture
}


const compositShader = new ShaderMaterial({
    uniforms: digitalGlitch.uniforms,
    vertexShader: digitalGlitch.vertexShader,
    fragmentShader: digitalGlitch.fragmentShader,
})


const composer = new EffectComposer(renderer);
const compositPass = new ShaderPass(compositShader);
composer.addPass(compositPass)


// Animating the glitch
setInterval(function () {
    digitalGlitch.uniforms.seed.value = randFloat(-1, 1)
    digitalGlitch.uniforms.seed_x.value = randFloat(-1, 1)
    digitalGlitch.uniforms.seed_y.value = randFloat(-1, 1)
    digitalGlitch.uniforms.tDisp.value = generateHeightmap(123)
}, 100)


function randFloat(min = -1, max = 1) {
    return Math.random() * (max - min) + min;
}


kbbRotate = true


let paused = false
// document.getElementById('pauseAnim').addEventListener('click', function () {
//     paused = !paused
// })


function toggleGlitch() {
    digitalGlitch.uniforms.byp.value === 1 ?
        digitalGlitch.uniforms.byp.value = 0 :
        digitalGlitch.uniforms.byp.value = 1
}

let glitchTillTimer = Math.random() * 32 + 5

function randomGlitch(eTime) {
    if (eTime > glitchTillTimer) {
        const duration = Math.random() * 300
        toggleGlitch()
        setTimeout(() => {
            toggleGlitch()
            glitchTillTimer = Math.random() * 52 + eTime
        }, duration)
    }
}


function playFall() {
    kbbRotate = !kbbRotate
    console.log(camTrgt.position)

    fallAnimArr.forEach(anim => {
        anim.paused = false
        anim.play()
    })
    engineFire.children.forEach(fire => fire.visible = !fire.visible)
    engineFireShadow.children.forEach(fire => fire.visible = !fire.visible)
    console.log(camTrgt.position)
}

function resetFall() {
    function glitchAnim(randDelay) {
        return new Promise((resolve) => {
            toggleGlitch()
            setTimeout(() => {
                toggleGlitch()
                resolve()
            }, randDelay)
        })
    }

    async function fallAnimReset(randDelay) {
        await glitchAnim(randDelay)
        fallAnimArr.forEach(anim => {
            anim.reset()
            anim.paused = true
        })

        kbbRotate = !kbbRotate

        engineFire.children.forEach(fire => fire.visible = !fire.visible)
        engineFireShadow.children.forEach(fire => fire.visible = !fire.visible)
    }

    const randGlitchDelay = Math.random() * 800 + 1
    fallAnimReset(randGlitchDelay)

}


document.getElementById('bottom-works-menupoint').addEventListener('click', playFall)
document.getElementById('close-about-btn').addEventListener('click', resetFall)


window.addEventListener('resize', function () {
    mql = window.matchMedia('(max-width: 640px)').matches
    renderer.setSize(window.innerWidth, window.innerHeight)
    camera.aspect = window.innerWidth / window.innerHeight
    camera.updateProjectionMatrix()

    // page resizing adjustments
    camera.lookAt(mql ? camTrgtMobile.position : camTrgt.position)
    camera.fov = mql ? 16 : 13
    mql ? shadowLight.position.set(1, 20, -9) : shadowLight.position.set(12, 22, 22)
})


function animate() {

    const delta = clock.getDelta()
    const elapsedTime = clock.elapsedTime

    // mixers update
    fireMixer.update(delta)
    kbbMixer.update(delta)
    shadowFireMixer.update(delta)
    shadowMixer.update(delta)
    engineMixer.update(delta)
    camTrgtMixer.update(delta)
    lampsMixer.update(delta)

    // random glitches
    randomGlitch(elapsedTime)


    // kebab rotation animation
    kbbRotate ? kbbRotationGroup.rotation.z -= 0.001 : null
    kbbRotate ? kbbRocketShadow.rotation.z -= 0.001 : null


    // camera target movement
    camera.lookAt(mql ? camTrgtMobile.position : camTrgt.position)

    // if (!kbbRotate) {
    //     camera.lookAt(mql ? camTrgt.position : camTrgtMobile.position)
    // }


    // compositing/rendering
    renderer.setRenderTarget(compositRenderTarget1)
    renderer.render(scene, camera)
    renderer.setRenderTarget(null)
    renderer.setRenderTarget(compositRenderTarget2)
    renderer.render(shadowScene, camera)
    renderer.setRenderTarget(null)

    composer.render()

    compositRenderTarget2.dispose()
    compositRenderTarget1.dispose()
}

const ls = document.getElementById('loadingScreen')

loadScene().then(() => {
    // playFall()
    renderer.setAnimationLoop(animate)
    ls.style.display = 'none'
}).catch((error) => {
    console.log("Error occurred in loading: ", error)
})

