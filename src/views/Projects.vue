<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { Modal } from 'bootstrap';

interface ProjectUser {
  id: string;
  firstName: string;
  lastName: string;
  nickname: string;
}

interface Project {
  id: string;
  name: string;
  tag: string;
  icon: string;
  color: string;
  description: string;
  createdAt: number;
  users: ProjectUser[];
}

const projects = ref<Project[]>([]);
const searchQuery = ref('');
const isEditing = ref(false);
const currentProject = ref<Project>({
  id: '',
  name: '',
  tag: '',
  icon: 'ti-package',
  color: '#3b82f6',
  description: '',
  createdAt: 0,
  users: []
});

const availableIcons = [
  'ti-package', 'ti-archive', 'ti-folder', 'ti-brand-github', 'ti-brand-docker',
  'ti-database', 'ti-globe', 'ti-device-laptop', 'ti-settings', 'ti-user',
  'ti-flame', 'ti-bolt', 'ti-heart', 'ti-star', 'ti-bug', 'ti-code',
  'ti-layout-kanban', 'ti-list-check', 'ti-calendar', 'ti-chart-bar'
];

const availableColors = [
  '#3b82f6', '#ef4444', '#10b981', '#f59e0b', '#6366f1', '#8b5cf6', '#ec4899', '#64748b'
];

onMounted(() => {
  const saved = localStorage.getItem('celerix-projects');
  if (saved) {
    try {
      projects.value = JSON.parse(saved);
    } catch (e) {
      console.error('Failed to load projects', e);
    }
  }
});

watch(projects, (newProjects) => {
  localStorage.setItem('celerix-projects', JSON.stringify(newProjects));
}, { deep: true });

const filteredProjects = computed(() => {
  if (!searchQuery.value) return projects.value;
  const query = searchQuery.value.toLowerCase();
  return projects.value.filter(p => 
    p.name.toLowerCase().includes(query) || 
    p.tag.toLowerCase().includes(query) ||
    p.description.toLowerCase().includes(query)
  );
});

const openProjectModal = (project?: Project) => {
  if (project) {
    isEditing.value = true;
    currentProject.value = { ...project };
  } else {
    isEditing.value = false;
    currentProject.value = {
      id: crypto.randomUUID(),
      name: '',
      tag: '',
      icon: 'ti-package',
      color: '#3b82f6',
      description: '',
      createdAt: Date.now(),
      users: []
    };
  }
  const modal = new Modal(document.getElementById('projectModal') as HTMLElement);
  modal.show();
};

const saveProject = () => {
  if (isEditing.value) {
    const index = projects.value.findIndex(p => p.id === currentProject.value.id);
    if (index !== -1) {
      projects.value[index] = { ...currentProject.value };
    }
  } else {
    projects.value.push({ ...currentProject.value });
  }
  const modalElement = document.getElementById('projectModal');
  const modal = Modal.getInstance(modalElement as HTMLElement);
  modal?.hide();
};

const deleteProject = (id: string) => {
  if (confirm('Are you sure you want to delete this project?')) {
    projects.value = projects.value.filter(p => p.id !== id);
    const modalElement = document.getElementById('projectModal');
    const modal = Modal.getInstance(modalElement as HTMLElement);
    modal?.hide();
  }
};

const addUser = () => {
  currentProject.value.users.push({
    id: crypto.randomUUID(),
    firstName: '',
    lastName: '',
    nickname: ''
  });
};

const removeUser = (index: number) => {
  currentProject.value.users.splice(index, 1);
};

</script>

<template>
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center justify-content-between w-100">
      <div class="d-flex align-items-center gap-2">
        <i class="ti ti-archive"></i> 
        <strong>Projects</strong>
      </div>
    </div>
  </teleport>

  <teleport to="#page-context">
    <div class="d-flex align-items-center gap-2" style="width: 300px;">
      <div class="search-input-group input-group input-group-sm">
        <span class="input-group-text bg-transparent border-end-0">
          <i class="ti ti-search"></i>
        </span>
        <input 
          type="text" 
          v-model="searchQuery" 
          class="form-control border-start-0 shadow-none" 
          placeholder="Filter projects..."
        >
      </div>
    </div>
  </teleport>

  <div class="container-fluid py-4">
    <div class="row mb-4">
      <div class="col-12">
        <div class="card bg-light-subtle border-0 shadow-sm">
          <div class="card-body">
            <h5 class="card-title">Manage Your Projects</h5>
            <p class="card-text text-muted">
              Projects help you organize and categorize your work across the application. 
              Assign tags to projects and use them to filter tasks in the Kanban board or other tools.
            </p>
          </div>
        </div>
      </div>
    </div>

    <div class="row g-4">
      <div v-for="project in filteredProjects" :key="project.id" class="col-12 col-md-6 col-lg-4 col-xl-3">
        <div 
          class="project-card card h-100 border-0 shadow-sm position-relative overflow-hidden" 
          @click="openProjectModal(project)"
          :style="{ borderTop: `4px solid ${project.color}` }"
        >
          <div class="card-body d-flex flex-column">
            <div class="d-flex align-items-center gap-3 mb-3">
              <div 
                class="project-icon-wrapper d-flex align-items-center justify-content-center rounded"
                :style="{ backgroundColor: project.color + '20', color: project.color }"
              >
                <i :class="['ti', project.icon, 'fs-3']"></i>
              </div>
              <div class="overflow-hidden">
                <h6 class="card-title mb-0 text-truncate">{{ project.name }}</h6>
                <span class="badge" :style="{ backgroundColor: project.color + '20', color: project.color }">
                  #{{ project.tag }}
                </span>
              </div>
            </div>
            <p class="card-text text-muted small flex-grow-1">
              {{ project.description || 'No description provided.' }}
            </p>
            <div class="mt-auto pt-3 border-top d-flex justify-content-between align-items-center">
              <span class="text-muted smaller">
                Created: {{ new Date(project.createdAt).toLocaleDateString() }}
              </span>
              <i class="ti ti-chevron-right text-muted"></i>
            </div>
          </div>
        </div>
      </div>

      <!-- Add Project Card -->
      <div v-if="!searchQuery" class="col-12 col-md-6 col-lg-4 col-xl-3">
        <div 
          class="add-project-card card h-100 border-0 shadow-sm d-flex flex-column justify-content-center align-items-center text-muted"
          style="min-height: 200px; cursor: pointer"
          @click="openProjectModal()"
        >
          <i class="ti ti-plus fs-1 mb-2"></i>
          <span>Add Project</span>
        </div>
      </div>

      <!-- Empty State for Search -->
      <div v-if="searchQuery && filteredProjects.length === 0" class="col-12 text-center py-5">
        <div class="text-muted">
          <i class="ti ti-package-off fs-1 mb-3 d-block"></i>
          <h5>No projects found</h5>
          <p>Try a different search term or clear the filter.</p>
        </div>
      </div>
    </div>
  </div>

  <!-- Project Modal -->
  <div class="modal fade" id="projectModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">{{ isEditing ? 'Edit Project' : 'New Project' }}</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="saveProject">
            <div class="mb-3">
              <label class="form-label">Project Name</label>
              <input v-model="currentProject.name" type="text" class="form-control" placeholder="e.g. Website Redesign" required>
            </div>
            <div class="mb-3">
              <label class="form-label">Tag Name</label>
              <div class="input-group">
                <span class="input-group-text">#</span>
                <input v-model="currentProject.tag" type="text" class="form-control" placeholder="e.g. web-dev" required>
              </div>
            </div>
            <div class="mb-3">
              <label class="form-label">Description</label>
              <textarea v-model="currentProject.description" class="form-control" rows="3" placeholder="Optional project details..."></textarea>
            </div>
            
            <div class="mb-3">
              <label class="form-label d-block">Icon & Color</label>
              <div class="d-flex flex-wrap gap-2 mb-3">
                <button 
                  v-for="color in availableColors" 
                  :key="color"
                  type="button"
                  class="color-btn rounded-circle border-0"
                  :style="{ backgroundColor: color, outline: currentProject.color === color ? '2px solid var(--bs-primary)' : 'none', outlineOffset: '2px' }"
                  @click="currentProject.color = color"
                ></button>
              </div>
              <div class="icon-picker p-2 bg-light rounded border d-flex flex-wrap gap-2">
                <button 
                  v-for="icon in availableIcons" 
                  :key="icon"
                  type="button"
                  class="btn btn-sm"
                  :class="currentProject.icon === icon ? 'btn-primary' : 'btn-light'"
                  @click="currentProject.icon = icon"
                  title="icon"
                >
                  <i :class="['ti', icon]"></i>
                </button>
              </div>
            </div>

            <div class="mb-3">
              <div class="d-flex justify-content-between align-items-center mb-2">
                <label class="form-label mb-0">Project Users</label>
                <button type="button" class="btn btn-sm btn-outline-primary" @click="addUser">
                  <i class="ti ti-user-plus"></i> Add User
                </button>
              </div>
              <div v-if="currentProject.users.length === 0" class="text-center py-3 bg-light rounded border border-dashed">
                <small class="text-muted">No users added to this project yet.</small>
              </div>
              <div v-else class="user-list">
                <div v-for="(user, index) in currentProject.users" :key="user.id" class="row g-2 mb-2 align-items-end border-bottom pb-2">
                  <div class="col-4">
                    <label class="smaller text-muted">First Name</label>
                    <input v-model="user.firstName" type="text" class="form-control form-control-sm" placeholder="First" required>
                  </div>
                  <div class="col-4">
                    <label class="smaller text-muted">Last Name</label>
                    <input v-model="user.lastName" type="text" class="form-control form-control-sm" placeholder="Last" required>
                  </div>
                  <div class="col-3">
                    <label class="smaller text-muted">Nickname</label>
                    <input v-model="user.nickname" type="text" class="form-control form-control-sm" placeholder="Nick" required>
                  </div>
                  <div class="col-1">
                    <button type="button" class="btn btn-sm btn-link text-danger p-0" @click="removeUser(index)">
                      <i class="ti ti-x fs-5"></i>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </form>
        </div>
        <div class="modal-footer justify-content-between">
          <div>
            <button v-if="isEditing" type="button" class="btn btn-outline-danger" @click="deleteProject(currentProject.id)">
              <i class="ti ti-trash"></i>
            </button>
          </div>
          <div class="d-flex gap-2">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
            <button type="button" class="btn btn-primary" @click="saveProject" :disabled="!currentProject.name || !currentProject.tag">
              {{ isEditing ? 'Update Project' : 'Create Project' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.project-card {
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.project-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 20px rgba(0,0,0,0.1) !important;
}

.add-project-card:hover {
  background-color: var(--bs-secondary-bg);
  transform: translateY(-5px);
}

.project-icon-wrapper {
  width: 48px;
  height: 48px;
  flex-shrink: 0;
}

.color-btn {
  width: 24px;
  height: 24px;
  padding: 0;
  transition: transform 0.1s;
}

.color-btn:hover {
  transform: scale(1.2);
}

.icon-picker {
  max-height: 150px;
  overflow-y: auto;
}

.smaller {
  font-size: 0.75rem;
}

.search-input-group {
  transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
  border: 1px solid var(--bs-border-color);
  border-radius: var(--bs-border-radius-sm);
}

.search-input-group:focus-within {
  border-color: #86b7fe;
  outline: 0;
}

.search-input-group .input-group-text,
.search-input-group .form-control {
  border: none;
}
.border-dashed {
  border-style: dashed !important;
}

.user-list {
  max-height: 200px;
  overflow-y: auto;
  padding-right: 5px;
}
</style>