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
                component: Dummy
            },
            {
                path: 'settings',
                name: 'settings',
                component: Dummy
            },
            {
                path: 'projects',
                name: 'projects',
                component: Projects
            }
        ]
    }
];
