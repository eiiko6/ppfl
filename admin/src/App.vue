<template>
  <div>
    <div v-if="!isAuthenticated" class="login-container">
      <h2>Admin Login</h2>
      <input v-model="loginUsername" placeholder="Username" class="login-input" />
      <input v-model="loginPassword" type="password" placeholder="Password" class="login-input" />
      <button @click="login" class="btn-login">Login</button>
      <p v-if="loginError" class="error">{{ loginError }}</p>
    </div>

    <main v-else class="admin-grid">
      <ImageCard v-for="(img, index) in images" :key="img.path" :image="img" :backend-url="backendUrl"
        @update:title="val => img.title = val" @update:description="val => img.description = val"
        @delete="deleteImage(index)" />

      <div class="upload-section">
        <label for="file-upload" class="btn-upload">Upload Images</label>
        <input id="file-upload" type="file" multiple accept="image/*" @change="onFilesSelected" style="display: none" />
      </div>

      <button @click="applyChanges" class="btn-apply" :disabled="loading">
        {{ loading ? 'Applying...' : 'Apply Changes' }}
      </button>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import axios from 'axios'
import ImageCard from './components/ImageCard.vue'

const backendUrl = 'http://localhost:8080/admin'
const loginUsername = ref('')
const loginPassword = ref('')
const isAuthenticated = ref(false)
const loginError = ref('')

const images = ref([])
const loading = ref(false)

function getAuthHeader() {
  return {
    Authorization: 'Basic ' + btoa(`${loginUsername.value}:${loginPassword.value}`)
  }
}

async function login() {
  try {
    const res = await axios.get(`${backendUrl}/images?ts=${Date.now()}`, {
      headers: getAuthHeader(),
    })
    images.value = res.data.map((img) => ({ ...img, _new: false }))
    isAuthenticated.value = true
    loginError.value = ''
  } catch (err) {
    loginError.value = 'Invalid credentials'
  }
}

function deleteImage(index) {
  images.value.splice(index, 1)
}

function onFilesSelected(event) {
  const files = Array.from(event.target.files)
  files.forEach((file) => {
    images.value.push({
      file,
      path: URL.createObjectURL(file),
      title: '',
      description: '',
      _new: true,
    })
  })
  event.target.value = ''
}

async function applyChanges() {
  loading.value = true
  const authHeader = getAuthHeader()

  for (const img of images.value.filter((i) => i._new)) {
    const formData = new FormData()
    formData.append('file', img.file)
    formData.append('title', img.title)
    formData.append('description', img.description)

    try {
      const res = await axios.post(`${backendUrl}/upload`, formData, {
        headers: {
          ...authHeader,
          'Content-Type': 'multipart/form-data'
        },
      })
      img.path = res.data.path
      img._new = false
      delete img.file
    } catch (e) {
      console.error('Upload failed for', img.file.name)
    }
  }

  const toSave = images.value.map(({ file, _new, ...rest }) => rest)

  try {
    await axios.post(`${backendUrl}/update`, toSave, {
      headers: {
        ...authHeader,
        'Content-Type': 'application/json'
      }
    })
  } catch (e) {
    console.error('Failed to update image list:', e)
  }

  try {
    const res = await axios.get(`${backendUrl}/images`, { headers: authHeader })
    images.value = res.data.map((img) => ({ ...img, _new: false }))
  } catch (e) {
    console.error('Failed to reload image list')
  }

  loading.value = false
}
</script>

<style scoped>
.admin-grid {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 20px;
  width: 100%;
  max-width: 1200px;
  margin: 15px auto 0;
  box-sizing: border-box;
}

.upload-section {
  width: 100%;
  max-width: 1200px;
  text-align: center;
  margin: 20px 0;
}

.btn-upload {
  background-color: #1976d2;
  color: white;
  padding: 0.6rem 1.2rem;
  border-radius: 0.4rem;
  cursor: pointer;
  user-select: none;
}

.btn-apply {
  background-color: #4caf50;
  color: white;
  padding: 1rem 2rem;
  border-radius: 0.5rem;
  cursor: pointer;
  width: 100%;
  max-width: 1200px;
  font-size: 1.2rem;
  margin: 10px auto 40px;
  display: block;
}

.login-container {
  max-width: 400px;
  margin: 100px auto;
  padding: 2rem;
  border: 1px solid #ddd;
  border-radius: 0.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.login-input {
  display: block;
  width: 100%;
  margin: 0.5rem 0;
  padding: 0.5rem;
  font-size: 1rem;
  border: 1px solid #ccc;
  border-radius: 0.3rem;
}

.btn-login {
  background-color: #1976d2;
  color: white;
  padding: 0.6rem 1.2rem;
  border-radius: 0.4rem;
  cursor: pointer;
  margin-top: 1rem;
  width: 100%;
  font-size: 1rem;
  border: none;
}

.error {
  color: red;
  margin-top: 1rem;
  font-weight: bold;
}
</style>
