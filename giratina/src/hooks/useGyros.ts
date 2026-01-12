import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { CreateGyrosVersionRequest } from '../types';

export const GYROS_QUERY_KEY = ['gyros-versions'];

export function useGyrosVersions() {
  return useQuery({
    queryKey: GYROS_QUERY_KEY,
    queryFn: () => api.getGyrosVersions(),
  });
}

export function useCreateGyrosVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (data: CreateGyrosVersionRequest) => api.createGyrosVersion(data),
    onSuccess: (newVersion) => {
      queryClient.invalidateQueries({ queryKey: GYROS_QUERY_KEY });
      message.success(`Gyros version ${newVersion.version} created successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create Gyros version';
      message.error(errorMessage);
    },
  });
}

export function useSetGyrosVersionCurrent() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.setGyrosVersionCurrent(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: GYROS_QUERY_KEY });
      message.success('Current Gyros version updated');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to set current version';
      message.error(errorMessage);
    },
  });
}

export function useDeleteGyrosVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.deleteGyrosVersion(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: GYROS_QUERY_KEY });
      message.success('Gyros version deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete Gyros version';
      message.error(errorMessage);
    },
  });
}
