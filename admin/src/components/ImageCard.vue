<template>
  <div class="image-card">
    <img :src="image._new ? image.path : fullUrl(image.path)" class="image" />

    <input v-model="localImage.title" class="input-title" placeholder="Title" />
    <textarea v-model="localImage.description" class="input-description" placeholder="Description"></textarea>

    <button @click="$emit('delete')" class="btn-delete">Delete</button>
  </div>
</template>

<script setup>
import { watch, reactive } from 'vue'

const props = defineProps({
  image: Object,
  backendUrl: String,
})

const emit = defineEmits(['update:title', 'update:description', 'delete'])

const fullUrl = (path) => `${props.backendUrl.replace('/admin', '')}${path}`

// Local copy to avoid mutating props directly
const localImage = reactive({
  title: props.image.title,
  description: props.image.description
})

watch(() => localImage.title, (newVal) => emit('update:title', newVal))
watch(() => localImage.description, (newVal) => emit('update:description', newVal))
</script>

<style scoped>
.image-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  border: 2px solid #ddd;
  border-radius: 0.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 10px;
  width: 200px;
}

.image {
  width: 100%;
  height: 12rem;
  object-fit: cover;
  border-radius: 0.25rem;
}

.input-title {
  margin-top: 0.5rem;
  width: 100%;
  font-weight: bold;
  font-size: 1rem;
  padding: 0.25rem 0.5rem;
}

.input-description {
  margin-top: 0.25rem;
  width: 100%;
  resize: vertical;
  min-height: 3rem;
  padding: 0.25rem 0.5rem;
  font-size: 0.9rem;
}

.btn-delete {
  margin-top: 0.5rem;
  background: #f44336;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 0.3rem;
  cursor: pointer;
  width: 100%;
}
</style>
