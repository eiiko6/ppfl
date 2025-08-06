<template>
  <main class="image-grid">
    <div v-for="img in images" :key="img.path" class="image-card">
      <img :src="fullUrl(img.path)" class="image" />
      <h2 class="title">{{ img.title }}</h2>
      <p class="description">{{ img.description }}</p>
    </div>
  </main>
</template>


<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

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

.image-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  border: 2px solid #ddd;
  border-radius: 0.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 0;
  overflow: hidden;
}

.image {
  width: 100%;
  height: 12rem;
  /* 48 * 0.25rem = 12rem */
  object-fit: cover;
}

.title {
  font-weight: bold;
  margin-top: 0.5rem;
}

.description {
  margin-top: 0.25rem;
}
</style>
