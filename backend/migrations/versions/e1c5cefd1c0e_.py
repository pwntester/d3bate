"""empty message

Revision ID: e1c5cefd1c0e
Revises: 51d6962345fc
Create Date: 2020-03-24 16:53:51.404555

"""
from alembic import op
import sqlalchemy as sa

# revision identifiers, used by Alembic.
revision = 'e1c5cefd1c0e'
down_revision = '51d6962345fc'
branch_labels = None
depends_on = None


def upgrade():
    # ### commands auto generated by Alembic - please adjust! ###
    op.create_table('chat_message',
                    sa.Column('id', sa.Integer(), nullable=False),
                    sa.Column('is_reply', sa.Boolean(), nullable=True),
                    sa.Column('reply_to', sa.Integer(), nullable=True),
                    sa.Column('created', sa.DateTime(), nullable=False),
                    sa.PrimaryKeyConstraint('id')
                    )
    op.create_table('chat_message_thread',
                    sa.Column('id', sa.Integer(), nullable=False),
                    sa.Column('start_message', sa.Integer(), nullable=False),
                    sa.Column('last_active', sa.DateTime(), nullable=False),
                    sa.Column('user_count', sa.Integer(), nullable=False),
                    sa.Column('club_id', sa.Integer(), nullable=False),
                    sa.ForeignKeyConstraint(['club_id'], ['club.id'], ),
                    sa.ForeignKeyConstraint(['start_message'], ['chat_message.id'], ),
                    sa.PrimaryKeyConstraint('id')
                    )
    # ### end Alembic commands ###


def downgrade():
    # ### commands auto generated by Alembic - please adjust! ###
    op.drop_table('chat_message_thread')
    op.drop_table('chat_message')
    # ### end Alembic commands ###
