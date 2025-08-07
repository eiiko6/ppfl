<template>
  <div class="image-card">
    <img :src="fullUrl(path)" class="image" />
    <h2 class="title">{{ title }}</h2>
    <p class="description">{{ description }}</p>
  </div>
</template>

<script setup>
const props = defineProps({
  title: String,
  description: String,
  path: String,
  fullUrl: Function
})
</script>

<style scoped>
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
