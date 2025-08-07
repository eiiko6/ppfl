<template>
  <div @contextmenu.prevent class="image-card">
    <canvas ref="canvas" class="image-canvas"></canvas>
    <div class="text">
      <h2 class="title">{{ title }}</h2>
      <p class="description">{{ description }}</p>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'

const props = defineProps({
  title: String,
  description: String,
  path: String,
  fullUrl: Function
})

const canvas = ref(null)

onMounted(() => {
  const ctx = canvas.value.getContext('2d')
  const img = new Image()
  img.crossOrigin = 'anonymous' // Prevent caching metadata
  img.onload = () => {
    const canvasEl = canvas.value
    canvasEl.width = canvasEl.clientWidth
    canvasEl.height = canvasEl.clientHeight

    const ctx = canvasEl.getContext('2d')

    const canvasRatio = canvasEl.width / canvasEl.height
    const imgRatio = img.width / img.height

    let sx, sy, sWidth, sHeight

    if (imgRatio > canvasRatio) {
      // Image is wider than canvas, crop sides
      sHeight = img.height
      sWidth = sHeight * canvasRatio
      sx = (img.width - sWidth) / 2
      sy = 0
    } else {
      // Image is taller than canvas, crop top/bottom
      sWidth = img.width
      sHeight = sWidth / canvasRatio
      sx = 0
      sy = (img.height - sHeight) / 2
    }

    ctx.clearRect(0, 0, canvasEl.width, canvasEl.height)
    ctx.drawImage(img, sx, sy, sWidth, sHeight, 0, 0, canvasEl.width, canvasEl.height)
  }
  img.src = props.fullUrl(props.path)
})
</script>

<style scoped>
.image-card {
  position: relative;
  width: 28rem;
  height: 30rem;
  background-color: #242424aa;
  border: 2px solid #ffffffaa;
  /* border-radius: 0.5rem; */
  overflow: hidden;
  user-select: none;
  transition: transform 0.2s ease;
}

.image-card:hover {
  transform: scale(1.03);
  border: 2px solid #ffffffff;
}

.image-canvas {
  width: 100%;
  height: 100%;
  display: block;
}

.text {
  position: absolute;
  bottom: 0;
  left: 0;
  backdrop-filter: blur(20px) brightness(0.8);
  width: 100%;
  height: 20%;
  z-index: 1;
  transition: bottom 0.2s ease;
  bottom: -20%;
}

.image-card:hover .text {
  bottom: 0;
}

.title,
.description {
  color: white;
  position: relative;
  z-index: 2;
}

.title {
  font-weight: bold;
  margin: 0.5rem;
}

.description {
  margin: 0 0.5rem 0.5rem;
}

@media screen and (max-width: 800px) {
  .text {
    bottom: 0;
  }
}
</style>
