import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { SnorlaxVersion, CreateSnorlaxVersionRequest } from '../types';

export const SNORLAX_QUERY_KEY = ['snorlax-versions'];

export function useSnorlaxVersions() {
  return useQuery({
    queryKey: SNORLAX_QUERY_KEY,
    queryFn: () => api.getSnorlaxVersions(),
  });
}

export function useCreateSnorlaxVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (data: CreateSnorlaxVersionRequest) => api.createSnorlaxVersion(data),
    onSuccess: (newVersion) => {
      queryClient.invalidateQueries({ queryKey: SNORLAX_QUERY_KEY });
      message.success(`Snorlax version ${newVersion.version} created successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create Snorlax version';
      message.error(errorMessage);
    },
  });
}

export function useSetSnorlaxVersionCurrent() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.setSnorlaxVersionCurrent(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: SNORLAX_QUERY_KEY });
      message.success('Current Snorlax version updated');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to set current version';
      message.error(errorMessage);
    },
  });
}

export function useDeleteSnorlaxVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.deleteSnorlaxVersion(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: SNORLAX_QUERY_KEY });
      message.success('Snorlax version deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete Snorlax version';
      message.error(errorMessage);
    },
  });
}
