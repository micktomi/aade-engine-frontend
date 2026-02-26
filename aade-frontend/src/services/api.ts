import axios from 'axios';
import type { ValidationReport, BatchFileResult } from '../types';

const API_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';

const api = axios.create({
  baseURL: API_URL,
});

export const checkHealth = async () => {
  const response = await api.get('/health/live');
  return response.data;
};

// Το backend περιμένει XML String στο body (Single File - Legacy but kept)
export const validateInvoiceXml = async (xmlContent: string): Promise<ValidationReport[]> => {
  const response = await api.post('/validate', xmlContent, {
    headers: {
      'Content-Type': 'application/xml',
    },
  });
  return response.data;
};

// Batch Upload
export const validateBatchXmls = async (files: FileList): Promise<BatchFileResult[]> => {
    const formData = new FormData();
    for (let i = 0; i < files.length; i++) {
        formData.append('files', files[i]);
    }

    const response = await api.post('/validate/batch', formData);
    return response.data;
};

export default api;
