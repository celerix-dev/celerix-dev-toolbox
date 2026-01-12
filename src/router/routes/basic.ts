import type { RouteRecordRaw } from 'vue-router';
import HomeView from '@/views/HomeView.vue';
import Base64Index from "@/views/Base64Index.vue";
import Dummy from "@/views/Dummy.vue";
import Dashboard from "@/views/Dashboard.vue";
import UUIDView from "@/views/UUIDView.vue";
import JWTView from "@/views/JWTView.vue";

import KanbanView from "@/views/KanbanView.vue";
import Terminal from "@/views/Terminal.vue";
import Projects from "@/views/Projects.vue";
import DataViewer from "@/views/DataViewer.vue";
import ProfileView from "@/views/ProfileView.vue";
import SettingsView from "@/views/SettingsView.vue";

// Define the routes for the generic routes
export const basicRoutes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'home',
        component: HomeView,
        children: [
            {
                path: '',
                name: 'dashboard',
                component: Dashboard
            },
            {
                path: '/kanban',
                name: 'kanban-index',
                component: KanbanView
            },
            {
                path: 'base64',
                name: 'base64-index',
                component: Base64Index,
            },
            {
                path: 'jwt',
                name: 'jwt-index',
                component: JWTView,
            },
            {
                path: 'uuid',
                name: 'uuid-index',
                component: UUIDView,
            },
            {
                path: 'json',
                name: 'json-index',
                component: Dummy,
            },
            {
                path: 'profile',
                name: 'profile',
                component: ProfileView
            },
            {
                path: 'settings',
                name: 'settings',
                component: SettingsView
            },
            {
                path: 'projects',
                name: 'projects',
                component: Projects
            },
            {
                path: 'data-viewer',
                name: 'data-viewer',
                component: DataViewer
            }
        ]
    },
    {
        path: '/terminal',
        name: 'terminal',
        component: Terminal
    }
];
