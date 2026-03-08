<script setup lang="ts">
import ApplicationHeader from '@/components/Basic/ApplicationHeader.vue';
import {useRoute} from 'vue-router';
import logo from '@/assets/logo.svg';
import {onMounted} from "vue";

import {useUserStore} from '@/stores/user';

const route = useRoute();
const userStore = useUserStore();

onMounted(async () => {
    if (!userStore.isInitialized) {
        await userStore.loadUser();
    }
});

const isLinkActive = (itemPath: string) => {
    const currentPath = route.path;
    const activeMenu = route.meta.activeMenu;

    // Exact match
    // Or the current route's meta says this is the parent
    // Or it's a sub-path (standard behavior)
    return currentPath === itemPath || activeMenu === itemPath;
};

</script>

<template>

    <div class="cx-layout-300-60">
        <aside>
            <aside class="surface sidebar stack g-3" tabindex="-1" id="donkers-sidebar">
                <div class="">

                <div class="mb-3">
                    <input type="text" class="cx-input cx-w-100" placeholder="Search"/>
                </div>
                <ul class="sidebar-nav" data-key="dashboard" aria-expanded="true">
                    <li>
                        <h6 class="text-muted text-bold sidebar-header">Getting started</h6>
                    </li>
                    <li>
                        <hr style="margin:0" class="sidebar-divider"/>
                    </li>
                    <li class="nav-item">
                        <router-link class="nav-link pt-3 pb-3" :to="{ name: 'dashboard' }">
                            <div class="d-flex"><i class="ti ti-dashboard" style="font-size: var(--s-4);"></i>
                                <div class="ps-2">Dashboard</div>
                            </div>
                        </router-link>
                    </li>
                    <li class="mt-3">
                        <h6 class="text-muted text-bold sidebar-header">Applications</h6>
                    </li>
                    <li>
                        <hr style="margin:0" class="sidebar-divider"/>
                    </li>
                    <li class="mt-3">
                        <h6 class="text-muted text-bold sidebar-header">Tools</h6>
                    </li>
                    <li>
                        <hr style="margin:0" class="sidebar-divider"/>
                    </li>
                    <li class="nav-item">
                        <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/base64') ? 'active' : '')"
                                     :to="{ name: 'base64-index' }">
                            <div class="d-flex"><i class="ti ti-file-code-2" style="font-size: var(--s-4);"></i>
                                <div class="ps-2">Base64 En/Decoder</div>
                            </div>
                        </router-link>
                    </li>
                    <li class="nav-item">
                        <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/jwt') ? 'active' : '')"
                                     :to="{ name: 'jwt-index' }">
                            <div class="d-flex"><i class="ti ti-password-fingerprint" style="font-size: var(--s-4);"></i>
                                <div class="ps-2">JWT Inspector</div>
                            </div>
                        </router-link>
                    </li>
                    <li class="nav-item">
                        <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/uuid') ? 'active' : '')"
                                     :to="{ name: 'uuid-index' }">
                            <div class="d-flex"><i class="ti ti-fidget-spinner" style="font-size: var(--s-4);"></i>
                                <div class="ps-2">UUID Generator</div>
                            </div>
                        </router-link>
                    </li>
                    <li class="nav-item">
                        <router-link :class="`nav-link pt-3 pb-3 ` + (isLinkActive('/json') ? 'active' : '')"
                                     :to="{ name: 'json-index' }">
                            <div class="d-flex"><i class="ti ti-file-neutral" style="font-size: var(--s-4);"></i>
                                <div class="ps-2">JSON Editor</div>
                            </div>
                        </router-link>
                    </li>
                </ul>
            </div>
            </aside>
        </aside>

        <header class="sticky-header justify-between p-3" style="height: 61px;z-index:1024;">
            <ApplicationHeader/>
        </header>

        <main class="p-4">
            <RouterView/>
        </main>
    </div>
    <!-- Header content -->


        <div class="pos-absolute d-flex justify-between align-center flex-grow"
             style="width:300px;bottom:10px;left:10px;">
            <div class="d-flex align-center">
                <img draggable="false" :src="logo" alt="Logo" width="32" height="32"
                     class=""/>
                <div class="ps-1">Celerix</div>
            </div>
            <div>
                &nbsp;
            </div>
        </div>

</template>
<style>
html {
    font-size:0.9rem;
}
.sidebar {
    height: 100vh;
    position: sticky;
    top: 0;
    padding: var(--s-3);
    overflow-y: auto;
    overflow-x: hidden;
    border-right: 1px solid var(--separator);

    li {
        list-style: none;
        padding:var(--s-1);
        margin-bottom: var(--s-2);
    }

    h6 {
        font-variant: all-petite-caps;
    }
    .nav-item {
        .active {
            color: var(--brand-800);
            font-weight: bold;
        }
    }
}
</style>
