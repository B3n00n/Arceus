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
  Badge,
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
import { AssignmentModal } from '../components/AssignmentModal';
import type { Assignment } from '../types';

const { Title } = Typography;

// Extended type to include names
interface AssignmentWithDetails extends Assignment {
  arcade_name?: string;
  game_name?: string;
}

export const AssignmentsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedAssignment, setSelectedAssignment] = useState<Assignment | undefined>();
  const [searchText, setSearchText] = useState('');

  const { data: assignments = [], isLoading, refetch } = useAssignments();
  const { data: arcades = [] } = useArcades();
  const { data: games = [] } = useGames();
  const createMutation = useCreateAssignment();
  const updateMutation = useUpdateAssignment();
  const deleteMutation = useDeleteAssignment();

  // Enrich assignments with names
  const enrichedAssignments: AssignmentWithDetails[] = useMemo(() => {
    return assignments.map((assignment) => ({
      ...assignment,
      arcade_name: arcades.find((a) => a.id === assignment.arcade_id)?.name || 'Unknown',
      game_name: games.find((g) => g.id === assignment.game_id)?.name || 'Unknown',
    }));
  }, [assignments, arcades, games]);

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
    },
    {
      title: 'Arcade',
      dataIndex: 'arcade_name',
      key: 'arcade_name',
      width: 200,
      render: (name: string) => <Tag color="blue">{name}</Tag>,
      sorter: (a, b) => (a.arcade_name || '').localeCompare(b.arcade_name || ''),
    },
    {
      title: 'Game',
      dataIndex: 'game_name',
      key: 'game_name',
      width: 200,
      render: (name: string) => <Tag color="purple">{name}</Tag>,
      sorter: (a, b) => (a.game_name || '').localeCompare(b.game_name || ''),
    },
    {
      title: 'Version Status',
      key: 'version_status',
      width: 200,
      render: (_, record) => {
        const isSynced = record.current_version_id === record.assigned_version_id;
        return (
          <Space direction="vertical" size="small">
            <div>
              <strong>Assigned:</strong> Version ID {record.assigned_version_id}
            </div>
            {record.current_version_id && (
              <div>
                <strong>Current:</strong> Version ID {record.current_version_id}
              </div>
            )}
            {!record.current_version_id && (
              <div style={{ color: '#999' }}>
                <strong>Current:</strong> Not installed
              </div>
            )}
            {isSynced ? (
              <Badge status="success" text="Synced" />
            ) : (
              <Badge status="processing" text="Pending Update" />
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
      width: 120,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Tooltip title="Edit Assignment">
            <Button
              type="text"
              icon={<EditOutlined />}
              onClick={() => handleEdit(record)}
              size="small"
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
            <Tooltip title="Delete">
              <Button
                type="text"
                danger
                icon={<DeleteOutlined />}
                size="small"
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
    <div>
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }}>
        <div>
          <Title level={2} style={{ margin: 0, marginBottom: 8 }}>
            Game Assignments
          </Title>
          <Space size="large">
            <span>
              Total: <strong>{stats.total}</strong>
            </span>
            <Badge status="success" text={`Synced: ${stats.synced}`} />
            <Badge status="processing" text={`Pending: ${stats.pending}`} />
          </Space>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
        >
          Create Assignment
        </Button>
      </Flex>

      <Card>
        <Flex gap="middle" style={{ marginBottom: 16 }} wrap="wrap">
          <Input
            placeholder="Search by arcade or game..."
            prefix={<SearchOutlined />}
            allowClear
            style={{ width: 300 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
          />
          <Tooltip title="Refresh">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
            />
          </Tooltip>
        </Flex>

        <Table
          columns={columns}
          dataSource={filteredAssignments}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `Total ${total} assignments`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50', '100'],
          }}
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
