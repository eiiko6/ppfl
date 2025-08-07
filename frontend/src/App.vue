<template>
  <main class="image-grid">
    <ImageCard v-for="img in images" :key="img.path" :title="img.title" :description="img.description" :path="img.path"
      :fullUrl="fullUrl" />
  </main>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'
import ImageCard from './components/ImageCard.vue'

const images = ref([])

const backendUrl = 'http://localhost:8080'
const fullUrl = (path) => `${backendUrl}${path}`

onMounted(async () => {
  const res = await axios.get(`${backendUrl}/images`)
  images.value = res.data
})
</script>

<style scoped>
main {
  display: flex;
  justify-content: center;
}

.image-grid {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 20px;
  width: 100%;
  max-width: 1200px;
  margin: 15px auto 0;
  box-sizing: border-box;
}

@media (min-width: 768px) {
  .image-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}
</style>
