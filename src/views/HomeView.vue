<script setup lang="ts">
import ApplicationHeader from '@/components/Basic/ApplicationHeader.vue';
import {useRoute} from 'vue-router';
import logo from '@/assets/logo.svg';
import {ref, onMounted, computed} from "vue";
import CelerixTerminal from "@/components/CelerixTerminal.vue";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import { useUserStore } from '@/stores/user';
import { storageService } from '@/services/storage';

const route = useRoute();
const userStore = useUserStore();
const terminalVisible = ref(false);

const projects = ref<any[]>([]);
const isLoadingProjects = ref(true);

onMounted(async () => {
  if (!userStore.isInitialized) {
    await userStore.loadUser();
  }
  await loadProjects();
});

const loadProjects = async () => {
  try {
    const data = await storageService.load<any>('PROJECTS');
    if (data && data.projects) {
      projects.value = data.projects;
    } else if (Array.isArray(data)) {
      projects.value = data;
    }
  } catch (e) {
    console.error('Failed to load projects in sidebar', e);
  } finally {
    isLoadingProjects.value = false;
  }
};

const activeProject = computed(() => {
  return projects.value.find(p => p.id === userStore.activeProjectId) || null;
});

const selectProject = (projectId: string | null) => {
  userStore.setActiveProject(projectId);
};

const isLinkActive = (itemPath: string) => {
  const currentPath = route.path;
  const activeMenu = route.meta.activeMenu;

  // Exact match
  // Or the current route's meta says this is the parent
  // Or it's a sub-path (standard behavior)
  return currentPath === itemPath || activeMenu === itemPath;
};

const openTerminalInWindow = async () => {
  const id = Math.random().toString(36).substring(7);

  const webview = new WebviewWindow(`term-${id}`, {
    url: '/terminal',
    title: 'Celerix Terminal',
    width: 900,
    height: 600,
    decorations: true,
    shadow: true,
  });

  await webview.once('tauri://created', async () => {
    await webview.setFocus();
  });
}
</script>

<template>
  <nav class="sidebar offcanvas-start offcanvas-md" tabindex="-1" id="donkers-sidebar">
    <div class="offcanvas-body">
      <div class="mb-4">
        <label class="sidebar-header d-block mb-2">Active Project</label>
        <div class="dropdown">
          <button 
            class="btn btn-outline-secondary w-100 d-flex align-items-center justify-content-between dropdown-toggle shadow-none" 
            type="button" 
            data-bs-toggle="dropdown" 
            data-bs-display="static"
            aria-expanded="false"
          >
            <div class="d-flex align-items-center gap-2 overflow-hidden">
              <i v-if="activeProject" :class="['ti', activeProject.icon]" :style="{ color: activeProject.color }"></i>
              <i v-else class="ti ti-layers-intersect text-muted"></i>
              <span class="text-truncate">{{ activeProject ? activeProject.name : 'All Projects' }}</span>
            </div>
          </button>
          <ul class="dropdown-menu w-100 shadow-sm">
            <li>
              <a class="dropdown-item d-flex align-items-center gap-2" href="#" @click.prevent="selectProject(null)">
                <i class="ti ti-layers-intersect text-muted"></i> All Projects
              </a>
            </li>
            <li><hr class="dropdown-divider"></li>
            <li v-for="project in projects" :key="project.id">
              <a class="dropdown-item d-flex align-items-center gap-2" href="#" @click.prevent="selectProject(project.id)">
                <i :class="['ti', project.icon]" :style="{ color: project.color }"></i>
                <span class="text-truncate">{{ project.name }}</span>
              </a>
            </li>
            <li v-if="projects.length > 0"><hr class="dropdown-divider"></li>
            <li>
              <router-link class="dropdown-item d-flex align-items-center gap-2 text-primary" :to="{ name: 'projects' }">
                <i class="ti ti-settings"></i> Manage Projects
              </router-link>
            </li>
          </ul>
        </div>
      </div>
      <div class="mb-3">
        <input type="text" class="form-control" placeholder="Search"/>
      </div>
      <ul class="sidebar-nav" data-key="dashboard" aria-expanded="true">
        <li>
          <h6 class="sidebar-header">Getting started</h6>
        </li>
        <li>
          <hr class="sidebar-divider"/>
        </li>
        <li class="nav-item">
          <router-link class="nav-link pt-3 pb-3" :to="{ name: 'dashboard' }">
            <div class="d-flex"><i class="ti ti-dashboard fs-4"></i> <div class="ps-2">Dashboard</div></div>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/projects') ? 'active' : '')" :to="{ name: 'projects' }">
            <div class="d-flex"><i class="ti ti-archive fs-4"></i> <div class="ps-2">Projects</div></div>
          </router-link>
        </li>
        <li class="mt-3">
          <h6 class="sidebar-header">Applications</h6>
        </li>
        <li>
          <hr class="sidebar-divider"/>
        </li>
        <li class="nav-item">
          <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/kanban') ? 'active' : '')" :to="{ name: 'kanban-index' }">
            <div class="d-flex"><i class="ti ti-layout-kanban fs-4"></i> <div class="ps-2">Kanban Board</div></div>
          </router-link>
        </li>
        <li class="mt-3">
          <h6 class="sidebar-header">Tools</h6>
        </li>
        <li>
          <hr class="sidebar-divider"/>
        </li>
        <li class="nav-item">
          <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/base64') ? 'active' : '')" :to="{ name: 'base64-index' }">
            <div class="d-flex"><i class="ti ti-file-code-2 fs-4"></i> <div class="ps-2">Base64 En/Decoder</div></div>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/jwt') ? 'active' : '')" :to="{ name: 'jwt-index' }">
            <div class="d-flex"><i class="ti ti-password-fingerprint fs-4"></i> <div class="ps-2">JWT Inspector</div></div>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/uuid') ? 'active' : '')" :to="{ name: 'uuid-index' }">
            <div class="d-flex"><i class="ti ti-fidget-spinner fs-4"></i> <div class="ps-2">UUID Generator</div></div>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/json') ? 'active' : '')" :to="{ name: 'json-index' }">
            <div class="d-flex"><i class="ti ti-file-neutral fs-4"></i> <div class="ps-2">JSON Editor</div></div>
          </router-link>
        </li>
      </ul>
    </div>
  </nav>

  <!-- Sidebar toggle -->
  <button type="button" data-bs-toggle="offcanvas" data-bs-target="#donkers-sidebar"
          class="position-absolute btn btn-secondary mt-3 m-1 d-md-none" style="z-index: 1021"><i
      class="ti ti-menu-2"></i></button>

  <!-- Header content -->
  <ApplicationHeader/>

  <div class="position-absolute d-flex justify-content-between align-items-center flex-grow-1" style="width:300px;bottom:10px;left:10px;">
    <div class="d-flex align-items-center">
      <img draggable="false" :src="logo" alt="Logo" width="32" height="32"
           class=""/>
      <div class="ps-1">Celerix</div>
    </div>
    <div>
      <div class="btn btn-secondary btn-sm" @click="terminalVisible = !terminalVisible"><i class="ti ti-terminal"></i></div>
      <div class="btn btn-secondary btn-sm" @click="openTerminalInWindow"><i class="ti ti-window"></i></div>
    </div>
  </div>

  <div class="p-2" style="height: calc(100% - 61px)">
    <RouterView/>
  </div>

  <div v-if="terminalVisible" class="terminal-overlay position-absolute" style="left:320px;right:0;bottom:0;">
    <CelerixTerminal />
  </div>
</template>
