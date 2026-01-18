import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { CreateArcadeRequest, UpdateArcadeRequest } from '../types';

export const ARCADES_QUERY_KEY = ['arcades'];

export function useArcades() {
  return useQuery({
    queryKey: ARCADES_QUERY_KEY,
    queryFn: () => api.getArcades(),
  });
}

export function useArcade(id: number | null) {
  return useQuery({
    queryKey: [...ARCADES_QUERY_KEY, id],
    queryFn: () => api.getArcade(id!),
    enabled: id !== null,
  });
}

export function useCreateArcade() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (data: CreateArcadeRequest) => api.createArcade(data),
    onSuccess: (newArcade) => {
      queryClient.invalidateQueries({ queryKey: ARCADES_QUERY_KEY });
      message.success(`Arcade "${newArcade.name}" created successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create arcade';
      message.error(errorMessage);
    },
  });
}

export function useUpdateArcade() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: UpdateArcadeRequest }) =>
      api.updateArcade(id, data),
    onSuccess: (updatedArcade) => {
      queryClient.invalidateQueries({ queryKey: ARCADES_QUERY_KEY });
      queryClient.invalidateQueries({ queryKey: [...ARCADES_QUERY_KEY, updatedArcade.id] });
      message.success(`Arcade "${updatedArcade.name}" updated successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to update arcade';
      message.error(errorMessage);
    },
  });
}

export function useDeleteArcade() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.deleteArcade(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ARCADES_QUERY_KEY });
      message.success('Arcade deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete arcade';
      message.error(errorMessage);
    },
  });
}
