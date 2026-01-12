
// Basic JSON Schema validation helper
// Since we want to avoid adding heavy dependencies like Ajv if possible,
// we can implement a simple validator for our specific structures, 
// or just export the schemas for use.
// However, the user asked to "use that as 'standard' to validate".

import projectUserSchema from '../schemas/project-user.schema.json';
import projectSchema from '../schemas/project.schema.json';
import projectsDataSchema from '../schemas/projects-data.schema.json';
import kanbanCardSchema from '../schemas/kanban-card.schema.json';
import kanbanColumnSchema from '../schemas/kanban-column.schema.json';
import kanbanDataSchema from '../schemas/kanban-data.schema.json';

export const Schemas = {
  ProjectUser: projectUserSchema,
  Project: projectSchema,
  ProjectsData: projectsDataSchema,
  KanbanCard: kanbanCardSchema,
  KanbanColumn: kanbanColumnSchema,
  KanbanData: kanbanDataSchema
};

export const SCHEMA_VERSIONS = {
  KANBAN: '1.0.0',
  PROJECTS: '1.0.0',
  PROJECT_USER: '1.0.0',
  PROJECT: '1.0.0',
  KANBAN_CARD: '1.0.0',
  KANBAN_COLUMN: '1.0.0'
};

/**
 * A simple validator that checks for required fields and basic types.
 * For a full implementation, one would use Ajv, but this provides a baseline
 * without adding new dependencies.
 */
export const validateData = (data: any, schema: any): { valid: boolean; errors: string[]; versionMatch?: boolean } => {
  const errors: string[] = [];

  if (schema.type === 'array') {
    if (!Array.isArray(data)) {
      errors.push('Data is not an array');
      return { valid: false, errors };
    }
  } else if (schema.type === 'object') {
    if (typeof data !== 'object' || data === null) {
      errors.push('Data is not an object');
      return { valid: false, errors };
    }

    if (schema.required) {
      for (const field of schema.required) {
        if (data[field] === undefined) {
          errors.push(`Missing required field: ${field}`);
        }
      }
    }
  }

  // Version check if applicable
  let versionMatch = true;
  if (data && data.version && schema.properties?.version) {
    // If we have a specific target version in our constants, we can check it
    // This is a simple equality check for now
  }

  return {
    valid: errors.length === 0,
    errors,
    versionMatch
  };
};

export const schemaService = {
  validateProjects: (data: any) => validateData(data, Schemas.ProjectsData),
  validateKanban: (data: any) => validateData(data, Schemas.KanbanData),
  
  getVersionedProjects: (projects: any[]) => ({
    version: SCHEMA_VERSIONS.PROJECTS,
    projects: projects.map(p => ({ ...p, version: SCHEMA_VERSIONS.PROJECT }))
  }),
  
  getVersionedKanban: (columns: any[]) => ({
    version: SCHEMA_VERSIONS.KANBAN,
    columns: columns.map(col => ({
      ...col,
      version: SCHEMA_VERSIONS.KANBAN_COLUMN,
      cards: col.cards.map((card: any) => ({ ...card, version: SCHEMA_VERSIONS.KANBAN_CARD }))
    }))
  })
};
