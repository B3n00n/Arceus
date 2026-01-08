import { useState, useMemo } from 'react';
import {
  Typography,
  Button,
  Table,
  Space,
  Input,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
  Tag,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
  SyncOutlined,
  CheckCircleOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import {
  useAssignments,
  useCreateAssignment,
  useUpdateAssignment,
  useDeleteAssignment,
} from '../hooks/useAssignments';
import { useArcades } from '../hooks/useArcades';
import { useGames } from '../hooks/useGames';
import { useAllGameVersions } from '../hooks/useGameVersions';
import { AssignmentModal } from '../components/AssignmentModal';
import type { Assignment } from '../types';
import { useResponsive } from '../hooks/useResponsive';

const { Title } = Typography;

// Extended type to include names and version strings
interface AssignmentWithDetails extends Assignment {
  arcade_name?: string;
  game_name?: string;
  assigned_version_string?: string;
  current_version_string?: string;
}

export const AssignmentsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedAssignment, setSelectedAssignment] = useState<Assignment | undefined>();
  const [searchText, setSearchText] = useState('');
  const { isMobile } = useResponsive();

  const { data: assignments = [], isLoading, refetch } = useAssignments();
  const { data: arcades = [] } = useArcades();
  const { data: games = [] } = useGames();
  const createMutation = useCreateAssignment();
  const updateMutation = useUpdateAssignment();
  const deleteMutation = useDeleteAssignment();

  // Get unique game IDs from assignments to fetch their versions
  const gameIds = useMemo(() => {
    return Array.from(new Set(assignments.map((a) => a.game_id)));
  }, [assignments]);

  const { data: allVersions = [] } = useAllGameVersions(gameIds);

  // Enrich assignments with names and version strings
  const enrichedAssignments: AssignmentWithDetails[] = useMemo(() => {
    return assignments.map((assignment) => ({
      ...assignment,
      arcade_name: arcades.find((a) => a.id === assignment.arcade_id)?.name || 'Unknown',
      game_name: games.find((g) => g.id === assignment.game_id)?.name || 'Unknown',
      assigned_version_string:
        allVersions.find((v) => v.id === assignment.assigned_version_id)?.version || 'Unknown',
      current_version_string: assignment.current_version_id
        ? allVersions.find((v) => v.id === assignment.current_version_id)?.version
        : undefined,
    }));
  }, [assignments, arcades, games, allVersions]);

  const filteredAssignments = useMemo(() => {
    return enrichedAssignments.filter(
      (assignment) =>
        assignment.arcade_name?.toLowerCase().includes(searchText.toLowerCase()) ||
        assignment.game_name?.toLowerCase().includes(searchText.toLowerCase())
    );
  }, [enrichedAssignments, searchText]);

  const handleCreate = () => {
    setModalMode('create');
    setSelectedAssignment(undefined);
    setModalOpen(true);
  };

  const handleEdit = (assignment: Assignment) => {
    setModalMode('edit');
    setSelectedAssignment(assignment);
    setModalOpen(true);
  };

  const handleDelete = async (id: number) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleModalSubmit = async (values: any) => {
    try {
      if (modalMode === 'create') {
        await createMutation.mutateAsync(values);
      } else if (selectedAssignment) {
        const { assigned_version_id } = values;
        await updateMutation.mutateAsync({
          id: selectedAssignment.id,
          data: { assigned_version_id },
        });
      }
      setModalOpen(false);
      setSelectedAssignment(undefined);
    } catch (error) {
      // Error handling is done in the mutation hooks
    }
  };

  const handleModalCancel = () => {
    setModalOpen(false);
    setSelectedAssignment(undefined);
  };

  const columns: ColumnsType<AssignmentWithDetails> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      sorter: (a, b) => a.id - b.id,
      render: (id: number) => (
        <span style={{ color: '#94a3b8', fontWeight: 500, fontSize: 13 }}>#{id}</span>
      ),
    },
    {
      title: 'Arcade',
      dataIndex: 'arcade_name',
      key: 'arcade_name',
      width: 200,
      render: (name: string) => (
        <Tag
          color="blue"
          style={{
            fontSize: 13,
            padding: '4px 12px',
            borderRadius: 6,
            fontWeight: 500,
          }}
        >
          {name}
        </Tag>
      ),
      sorter: (a, b) => (a.arcade_name || '').localeCompare(b.arcade_name || ''),
    },
    {
      title: 'Game',
      dataIndex: 'game_name',
      key: 'game_name',
      width: 200,
      render: (name: string) => (
        <Tag
          color="purple"
          style={{
            fontSize: 13,
            padding: '4px 12px',
            borderRadius: 6,
            fontWeight: 500,
          }}
        >
          {name}
        </Tag>
      ),
      sorter: (a, b) => (a.game_name || '').localeCompare(b.game_name || ''),
    },
    {
      title: 'Version Status',
      key: 'version_status',
      width: 200,
      render: (_, record) => {
        return (
          <Space orientation="vertical" size={4} style={{ width: '100%' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span style={{ color: '#666', fontSize: '12px', minWidth: '65px' }}>Assigned:</span>
              <Tag color="blue" style={{ margin: 0, fontWeight: 500, fontSize: '12px', padding: '2px 8px' }}>
                v{record.assigned_version_string}
              </Tag>
            </div>
            {record.current_version_string ? (
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <span style={{ color: '#666', fontSize: '12px', minWidth: '65px' }}>Current:</span>
                <Tag color="green" style={{ margin: 0, fontWeight: 500, fontSize: '12px', padding: '2px 8px' }}>
                  v{record.current_version_string}
                </Tag>
              </div>
            ) : (
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <span style={{ color: '#666', fontSize: '12px', minWidth: '65px' }}>Current:</span>
                <Tag color="default" style={{ margin: 0, fontSize: '12px', padding: '2px 8px' }}>
                  Not installed
                </Tag>
              </div>
            )}
          </Space>
        );
      },
    },
    {
      title: 'Sync Status',
      key: 'sync_icon',
      width: 100,
      align: 'center',
      render: (_, record) => {
        const isSynced = record.current_version_id === record.assigned_version_id;
        return isSynced ? (
          <CheckCircleOutlined style={{ fontSize: 20, color: '#16a34a' }} />
        ) : (
          <SyncOutlined spin style={{ fontSize: 20, color: '#ea580c' }} />
        );
      },
      sorter: (a, b) => {
        const aSynced = a.current_version_id === a.assigned_version_id;
        const bSynced = b.current_version_id === b.assigned_version_id;
        return Number(aSynced) - Number(bSynced);
      },
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 140,
      fixed: 'right',
      align: 'center',
      render: (_, record) => (
        <Space size={8}>
          <Tooltip title="Edit Assignment">
            <Button
              type="default"
              icon={<EditOutlined />}
              onClick={() => handleEdit(record)}
              size="middle"
              style={{
                borderRadius: 6,
                borderColor: '#1e3a8a',
              }}
            />
          </Tooltip>
          <Popconfirm
            title="Delete assignment?"
            description="The arcade will no longer have access to this game."
            onConfirm={() => handleDelete(record.id)}
            okText="Delete"
            okButtonProps={{ danger: true }}
            cancelText="Cancel"
          >
            <Tooltip title="Delete Assignment">
              <Button
                danger
                icon={<DeleteOutlined />}
                size="middle"
                style={{
                  borderRadius: 6,
                }}
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const stats = useMemo(() => {
    const synced = enrichedAssignments.filter(
      (a) => a.current_version_id === a.assigned_version_id
    ).length;
    const pending = enrichedAssignments.length - synced;
    return { synced, pending, total: enrichedAssignments.length };
  }, [enrichedAssignments]);

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header Section */}
      <Flex
        justify="space-between"
        align="flex-start"
        style={{ marginBottom: 24 }}
        wrap="wrap"
        gap={16}
        vertical={isMobile}
      >
        <div style={{ width: isMobile ? '100%' : 'auto' }}>
          <Title
            level={2}
            style={{
              margin: 0,
              marginBottom: 12,
              fontSize: isMobile ? 22 : 28,
              fontWeight: 600,
            }}
          >
            Game Assignments
          </Title>
          <Flex gap={12} wrap="wrap">
            <Card
              size="small"
              style={{
                minWidth: 90,
                background: 'linear-gradient(135deg, #0a0a0a 0%, #1a1a2e 100%)',
                borderColor: '#1e3a8a',
                boxShadow: '0 2px 8px rgba(30, 58, 138, 0.15)',
              }}
              styles={{ body: { padding: '12px' } }}
            >
              <div style={{ textAlign: 'center' }}>
                <div style={{ fontSize: 18, fontWeight: 700, color: '#3b82f6' }}>{stats.total}</div>
                <div style={{ fontSize: 11, color: '#64748b', marginTop: 2 }}>Total</div>
              </div>
            </Card>
            <Card
              size="small"
              style={{
                minWidth: 90,
                background: 'linear-gradient(135deg, #0a0a0a 0%, #1a2e1a 100%)',
                borderColor: '#10b981',
                boxShadow: '0 2px 8px rgba(16, 185, 129, 0.15)',
              }}
              styles={{ body: { padding: '12px' } }}
            >
              <div style={{ textAlign: 'center' }}>
                <div style={{ fontSize: 18, fontWeight: 700, color: '#10b981' }}>{stats.synced}</div>
                <div style={{ fontSize: 11, color: '#64748b', marginTop: 2 }}>Synced</div>
              </div>
            </Card>
            <Card
              size="small"
              style={{
                minWidth: 90,
                background: 'linear-gradient(135deg, #0a0a0a 0%, #2e1f0a 100%)',
                borderColor: '#f59e0b',
                boxShadow: '0 2px 8px rgba(245, 158, 11, 0.15)',
              }}
              styles={{ body: { padding: '12px' } }}
            >
              <div style={{ textAlign: 'center' }}>
                <div style={{ fontSize: 18, fontWeight: 700, color: '#f59e0b' }}>{stats.pending}</div>
                <div style={{ fontSize: 11, color: '#64748b', marginTop: 2 }}>Pending</div>
              </div>
            </Card>
          </Flex>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
          style={{
            minHeight: 42,
            width: isMobile ? '100%' : 'auto',
          }}
        >
          Create Assignment
        </Button>
      </Flex>

      {/* Main Content Card */}
      <Card
        style={{
          borderRadius: 12,
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.2), 0 2px 4px -2px rgb(0 0 0 / 0.2)',
        }}
      >
        {/* Search Bar */}
        <Flex gap={12} style={{ marginBottom: 20 }} wrap="wrap" align="center">
          <Input
            placeholder="Search by arcade or game name..."
            prefix={<SearchOutlined style={{ color: '#3b82f6' }} />}
            allowClear
            style={{ maxWidth: isMobile ? '100%' : 400, flex: 1 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            size="large"
          />
          <Tooltip title="Refresh Data">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
              size="large"
              style={{ width: isMobile ? '100%' : 'auto' }}
            >
              Refresh
            </Button>
          </Tooltip>
        </Flex>

        {/* Data Table */}
        <Table
          columns={columns}
          dataSource={filteredAssignments}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `${total} assignment${total !== 1 ? 's' : ''} total`,
            showSizeChanger: !isMobile,
            pageSizeOptions: ['10', '20', '50', '100'],
            style: { marginTop: 16 },
            simple: isMobile,
          }}
          scroll={{ x: isMobile ? 800 : 1000 }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>

      <AssignmentModal
        open={modalOpen}
        mode={modalMode}
        assignment={selectedAssignment}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={createMutation.isPending || updateMutation.isPending}
      />
    </div>
  );
};
